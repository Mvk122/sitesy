use std::path::PathBuf;
use tera::Tera;
use walkdir::WalkDir;
use std::fs;

pub fn render_with_tera(
    individually_tagged_html: &Vec<(PathBuf, String)>,
    tera_config: tera::Context,
    reusable_components: Vec<(String, String)>
) {
    let mut tera = Tera::default();

    let tera_templates = individually_tagged_html
        .iter()
        .map(|(path, contents)| (path.to_string_lossy().to_string(), contents.to_owned()));
    
    tera.add_raw_templates(tera_templates).unwrap();
    tera.add_raw_templates(reusable_components).unwrap(); 

    for individual_html in individually_tagged_html {
        fs::write(&individual_html.0, tera.render(&individual_html.0.to_string_lossy().to_string(), &tera_config).unwrap()).unwrap();
    }
}

pub fn load_reusable_components(reusable_components_path: PathBuf) -> Vec<(String, String)> {
    let mut reusable_components: Vec<(String, String)> = vec![];

    for entry in WalkDir::new(&reusable_components_path).into_iter().skip(1) {
        let entry = entry.unwrap();
        let friendly_name = entry.path().strip_prefix(&reusable_components_path).unwrap().to_string_lossy().to_string();
        if entry.file_type().is_file() {
            reusable_components.push((friendly_name, fs::read_to_string(entry.path()).unwrap()))
        }
    }
    return reusable_components;
}
