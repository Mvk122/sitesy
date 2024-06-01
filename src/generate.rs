use std::error::Error;
use std::fs;
use std::path::PathBuf;

use log::warn;
use pulldown_cmark::{TextMergeStream, Options, Parser, };
use walkdir::WalkDir;

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

    for entry in WalkDir::new(&markdown_files_folder).into_iter().skip(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut write_path = out_path.join(path.strip_prefix(&markdown_files_folder).unwrap());

        if entry.file_type().is_dir() {
            fs::create_dir_all(write_path).unwrap();
        } else if entry.file_type().is_file() && path.extension().unwrap() == "md" {
            let entry_contents = fs::read_to_string(path).unwrap();
            let html_contents_string = generate_html_contents(entry_contents).unwrap();

            write_path.set_extension("html");
            fs::write(write_path, html_contents_string).unwrap();
        }
    }

    return Ok("Static Site Generation Complete!".to_string());
}

fn generate_html_contents(file_contents: String) -> Result<String, Box<dyn Error>> {
    let options = Options::from_bits_truncate(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS.bits());
    let parser = Parser::new_ext(file_contents.as_str(), options);

    let iterator = TextMergeStream::new(parser);

    for event in iterator {
        println!("{:?}", event);
    }

    return Ok(String::from("Test output"));
}
