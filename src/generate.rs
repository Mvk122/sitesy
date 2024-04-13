use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::utils::ReadDirIter;

struct MetaData {
    title: Option<String>,
    permalink: Option<String>,
}

struct MarkdownFile {
    metadata: MetaData,
    file_path: PathBuf,
}

fn get_all_metadata(markdown_files: &PathBuf) -> Vec<MarkdownFile> {
    let mut mappings = Vec::new();

    let markdown_files_path = ReadDirIter::new(&markdown_files)
        .expect(&format!("Could not read from {}", markdown_files.display()));

    for markdown_file in markdown_files_path {
        let file_path = markdown_file.path();
        let file_contents = fs::read_to_string(&file_path)
            .expect(&format!("Could not read file {}", file_path.display()));

        let metadata = get_metadata_from_file(file_contents);
        let markdown_file = MarkdownFile {
            metadata: metadata,
            file_path: file_path,
        };
        mappings.push(markdown_file);
    }

    return mappings;
}

fn get_metadata_from_file(file_contents: String) -> MetaData {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let parser = Parser::new_ext(file_contents.as_str(), options);

    let mut metadata = MetaData {
        title: None,
        permalink: None,
    };

    let mut in_metadata_block = false;
    for event in parser {
        // println!("{:?}", event);
        if let Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) = event {
            in_metadata_block = true;
        }
        if let Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) = event {
            in_metadata_block = false;
        }

        if in_metadata_block {
            if let Event::Text(text) = event {
                let parts: Vec<&str> = text.split(':').collect();

                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = String::from(parts[1].trim());

                    match key {
                        _ if key == "title" => metadata.title = Some(value),
                        _ if key == "permalink" => metadata.permalink = Some(value),
                        _ => {}
                    }
                }
            }
        }
    }
    return metadata;
}

pub fn generate(src_path: PathBuf, _output_path: PathBuf) -> Result<String, String> {
    if !(src_path.exists() && src_path.is_dir()) {
        return Err("Source path does not exist.".to_string());
    }

    let mut markdown_files_folder = src_path.clone();
    markdown_files_folder.push("md");

    let markdown_files = get_all_metadata(&markdown_files_folder);

    for markdown_file in markdown_files {
        println!("{}", markdown_file.metadata.title.unwrap());
    }

    return Ok("Static Site Generation Complete!".to_string());
}
