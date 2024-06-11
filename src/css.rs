use std::{fs, path::PathBuf};

use walkdir::WalkDir;

pub fn concatenate_css(src_path: &PathBuf, out_path: &PathBuf) {
    let mut concatenated_css = String::new();

    for entry in WalkDir::new(&src_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().is_file() && path.extension().unwrap() == "css" {
            concatenated_css.push_str(&fs::read_to_string(path).unwrap());
            concatenated_css.push('\n');
        }
    }
    fs::write(out_path, concatenated_css).unwrap();
}
