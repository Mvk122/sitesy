use pulldown_cmark::Event;
use std::{collections::BTreeMap, fs, path::PathBuf};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};


use crate::iterator::get_pulldown_cmark_iterator;

pub fn extract_all_frontmatter(
    markdown_files_folder: PathBuf,
) -> BTreeMap<PathBuf, BTreeMap<String, String>> {
    let mut frontmatter: BTreeMap<PathBuf, BTreeMap<String, String>> = BTreeMap::new();

    WalkDir::new(&markdown_files_folder)
        .into_iter()
        .skip(1)
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .for_each(|entry| extract_single_frontmatter(entry, &mut frontmatter));

    return frontmatter;
}

pub fn extract_single_frontmatter(
    markdown_file: walkdir::DirEntry,
    context_map: &mut BTreeMap<PathBuf, BTreeMap<String, String>>,
) {
    let entry_contents = fs::read_to_string(markdown_file.path()).unwrap();
    let iterator = get_pulldown_cmark_iterator(&entry_contents);
    let mut in_frontmatter = false;
    let mut context = BTreeMap::new();

    for event in iterator {
        match event {
            Event::Start(pulldown_cmark::Tag::MetadataBlock(
                pulldown_cmark::MetadataBlockKind::YamlStyle,
            )) => {
                in_frontmatter = true;
            }
            Event::End(pulldown_cmark::TagEnd::MetadataBlock(
                pulldown_cmark::MetadataBlockKind::YamlStyle,
            )) => {
                break;
            }
            Event::Text(text) => {
                if in_frontmatter {
                    for line in text.lines() {
                        if let Some((key, value)) = line.split_once(':') {
                            context.insert(key.trim().to_owned(), value.trim().to_owned());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    let last_changed: DateTime<Utc> = markdown_file.metadata().unwrap().modified().unwrap().into();
    context.insert(String::from("last_modified"), last_changed.format("%B %d %Y").to_string());

    context_map.insert(markdown_file.path().to_path_buf(), context);
}

pub fn extract_html_frontmatter(
    markdown_files_folder: PathBuf,
    out_path: PathBuf,
) -> BTreeMap<PathBuf, BTreeMap<String, String>> {
    let frontmatter = extract_all_frontmatter(markdown_files_folder.clone());
    let mut html_frontmatter = BTreeMap::new();

    for (old_key, value) in frontmatter {
        let new_key = out_path
            .join(old_key.strip_prefix(&markdown_files_folder).unwrap())
            .with_extension("html")
            .to_path_buf();
        html_frontmatter.insert(new_key, value);
    }
    return html_frontmatter;
}
