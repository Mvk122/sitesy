use std::path::PathBuf;
use tera::Tera;
use std::fs;

pub fn render_with_tera(
    individually_tagged_html: &Vec<(PathBuf, String)>,
    tera_config: tera::Context,
) {
    let mut tera = Tera::default();

    let tera_templates = individually_tagged_html
        .iter()
        .map(|(path, contents)| (path.to_string_lossy().to_string(), contents.to_owned()));
    
    tera.add_raw_templates(tera_templates.clone()).unwrap();

    for (individual_html, tera_template) in individually_tagged_html.iter().zip(tera_templates) {

        fs::write(&individual_html.0, tera.render(&tera_template.0, &tera_config).unwrap()).unwrap();
    }
}
