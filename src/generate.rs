use pulldown_cmark::{html, CowStr::Borrowed, Event, Options, Parser, TextMergeStream};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use log::warn;
use walkdir::WalkDir;

use crate::{
    css::concatenate_css,
    load_config::create_tera_config,
    template_matching::{match_tag, match_tag_end},
    tera_funcs::{load_reusable_components, render_with_tera},
};

#[derive(Debug)]
struct HTMLTemplate {
    pre: String,
    post: String,
}

// Returns a Result where the type for the Ok value and the type for the Error values are Strings
pub fn generate(src_path: &PathBuf, out_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    validate_paths(&src_path, &out_path)?;
    _ = fs::create_dir_all(&out_path);

    let markdown_files_folder = src_path.join("md");

    let templates_map = load_templates(src_path.join("html").join("templates"));
    let reusable_components = load_reusable_components(src_path.join("html").join("components"));

    concatenate_css(
        &src_path.join("html").join("css"),
        &out_path.join("styles.css"),
    );
    let individual_tags = generate_html(markdown_files_folder, out_path, templates_map);

    let tera_config = create_tera_config(src_path);
    render_with_tera(&individual_tags, tera_config, reusable_components);

    return Ok("Static Site Generation Complete!".to_string());
}

fn validate_paths(src_path: &PathBuf, out_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !(src_path.exists() && src_path.is_dir()) {
        return Err("Source path does not exist.")?;
    }
    Ok(if out_path.exists() && out_path.is_dir() {
        warn!(
            "Output path {} already exists, overriding",
            out_path.to_string_lossy()
        );
        _ = fs::remove_dir_all(out_path);
    })
}

/// Returns the intermediate HTML in a dict where the first element is the path to write to and the second is the HTML
/// and copies the directory structure to the output folder
fn generate_html(
    markdown_files_folder: PathBuf,
    out_path: &PathBuf,
    templates_map: HashMap<String, HTMLTemplate>,
) -> Vec<(std::path::PathBuf, String)> {
    let mut individual_tags: Vec<(std::path::PathBuf, String)> = vec![];
    for entry in WalkDir::new(&markdown_files_folder).into_iter().skip(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        let write_path = out_path.join(path.strip_prefix(&markdown_files_folder).unwrap());

        if entry.file_type().is_dir() {
            // Copy the directory structure over
            fs::create_dir_all(write_path).unwrap();
            continue;
        }

        if entry.file_type().is_file() {
            if path.extension().unwrap() == "md" {
                // Converts the markdown elements into their exact html components as specified in html/templates
                // or the default provided by pulldown-cmark if that is not available
                let entry_contents = fs::read_to_string(path).unwrap();
                individual_tags.push((
                    write_path.with_extension("html").to_path_buf(),
                    generate_html_contents(entry_contents, &templates_map).unwrap(),
                ));
            } else {
                // Fallback to regular copying for non markdown files
                fs::copy(path, write_path).unwrap();
            }
        }
    }

    return individual_tags;
}

fn load_templates(templates_path: PathBuf) -> HashMap<String, HTMLTemplate> {
    let mut map = HashMap::new();

    for entry in WalkDir::new(&templates_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let template_contents = fs::read_to_string(path).unwrap();
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

struct TemplateMatch {
    template_name: String,
    is_start: bool,
}

fn match_event_to_template(event: &Event) -> Option<TemplateMatch> {
    return match event {
        Event::Start(tag) => match_tag(&tag).map(|tag_match| TemplateMatch {
            template_name: tag_match,
            is_start: true,
        }),
        Event::End(tag_end) => match_tag_end(&tag_end).map(|tag_match| TemplateMatch {
            template_name: tag_match,
            is_start: false,
        }),
        _ => None,
    };
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
        match event {
            Event::Text(Borrowed(text)) => {
                result.push_str(text);
            }
            _ => {
                if let Some(template_match) = match_event_to_template(&event) {
                    if let Some(template) = templates_map.get(&template_match.template_name) {
                        let text = if template_match.is_start {
                            &template.pre
                        } else {
                            &template.post
                        };
                        result.push_str(text);
                        continue;
                    }
                }
                // Use the pulldown_cmark default if a template can't be found
                let single_event_iter = std::iter::once(event);
                html::push_html(&mut result, single_event_iter);
            }
        }
    }
    return Ok(result);
}
