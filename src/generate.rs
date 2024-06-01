use pulldown_cmark::{
    html, CowStr::Borrowed, Event, HeadingLevel, Options, Parser, Tag, TagEnd, TextMergeStream,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, iter};

use log::warn;
use walkdir::WalkDir;

#[derive(Debug)]
struct HTMLTemplate {
    pre: String,
    post: String,
}

// Returns a Result where the type for the Ok value and the type for the Error values are Strings
pub fn generate(src_path: PathBuf, out_path: PathBuf) -> Result<String, Box<dyn Error>> {
    if !(src_path.exists() && src_path.is_dir()) {
        return Err("Source path does not exist.")?;
    }

    if out_path.exists() && out_path.is_dir() {
        warn!(
            "Output path {} already exists, overriding",
            out_path.to_string_lossy()
        );
        _ = fs::remove_dir_all(&out_path);
    }

    let markdown_files_folder = src_path.join("md");

    _ = fs::create_dir_all(&out_path);

    let templates_map = load_templates(src_path.join("html").join("templates"));

    for entry in WalkDir::new(&markdown_files_folder).into_iter().skip(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut write_path = out_path.join(path.strip_prefix(&markdown_files_folder).unwrap());

        if entry.file_type().is_dir() {
            fs::create_dir_all(write_path).unwrap();
        } else if entry.file_type().is_file() {
            if path.extension().unwrap() == "md" {
                let entry_contents = fs::read_to_string(path).unwrap();
                let html_contents_string =
                    generate_html_contents(entry_contents, &templates_map).unwrap();

                write_path.set_extension("html");
                fs::write(write_path, html_contents_string).unwrap();
            } else {
                // Fallback to regular copying for non markdown files
                fs::copy(path, write_path).unwrap();
            }
        }
    }

    return Ok("Static Site Generation Complete!".to_string());
}

fn load_templates(templates_path: PathBuf) -> HashMap<String, HTMLTemplate> {
    let mut map = HashMap::new();

    for entry in WalkDir::new(&templates_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let template_contents = fs::read_to_string(path).unwrap(); // Error handling recommended
            let parts: Vec<&str> = template_contents.split("{{ contents }}").collect();

            if parts.len() == 2 {
                let template_object = HTMLTemplate {
                    pre: parts[0].to_string(),
                    post: parts[1].to_string(),
                };

                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    map.insert(stem.to_string(), template_object);
                }
            }
        }
    }
    return map;
}

fn match_event_to_template(event: Event) -> (Option<String>, Option<bool>) {
    if let Event::Start(Tag::Heading { level, .. }) = event {
        return (Some(format!("h{}", level as u8)), Some(true));
    }
    if let Event::End(TagEnd::Heading(level, ..)) = event {
        return (Some(format!("h{}", level as u8)), Some(false));
    }
    if let Event::Start(Tag::Paragraph) = event {
        return (Some(String::from("p")), Some(true));
    }
    if let Event::End(TagEnd::Paragraph) = event {
        return (Some(String::from("p")), Some(false));
    }

    return (None, None);
}

fn generate_html_contents(
    file_contents: String,
    templates_map: &HashMap<String, HTMLTemplate>,
) -> Result<String, Box<dyn Error>> {
    let options = Options::from_bits_truncate(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS.bits());
    let parser = Parser::new_ext(file_contents.as_str(), options);

    let iterator = TextMergeStream::new(parser);

    let mut result = String::from("");
    for event in iterator {
        if let Event::Text(Borrowed(text)) = event {
            result.push_str(text) // Just push the raw text if its text
        } else {
            let (template_string, start_or_end) = match_event_to_template(event.clone());

            if let (Some(template_string), Some(start_or_end)) = (template_string, start_or_end) {
                match templates_map.get(&template_string) {
                    Some(template) => {
                        if start_or_end {
                            result.push_str(&template.pre);
                        } else {
                            result.push_str(&template.post);
                        }
                    }
                    None => {
                        let single_event_iter = std::iter::once(event);
                        html::push_html(&mut result, single_event_iter);
                    }
                }
            } else {
                let single_event_iter = std::iter::once(event);
                html::push_html(&mut result, single_event_iter);
            }
        }
    }

    return Ok(result);
}
