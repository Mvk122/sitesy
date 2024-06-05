use std::fs;
use std::path::PathBuf;
use log::info;


pub fn create_new_ssg_project(out_path: PathBuf) {
    fs::create_dir_all(&out_path).unwrap();
    fs::create_dir_all(&out_path.join("html")).unwrap();
    fs::create_dir_all(&out_path.join("html").join("templates")).unwrap();
    fs::create_dir_all(&out_path.join("html").join("css")).unwrap();
    fs::create_dir_all(&out_path.join("html").join("components")).unwrap();

    fs::create_dir_all(&out_path.join("md")).unwrap();

    let config_contents = r#"[vars]
author = "Your Name"
"#;

    fs::write(&out_path.join("config.toml"), config_contents).unwrap();

    info!("Success: Created blank repository for your new website!");
}
