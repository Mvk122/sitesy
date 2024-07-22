use std::{fs, path::PathBuf};

use toml::Table;

pub fn load_config(src_path: &PathBuf) -> toml::map::Map<std::string::String, toml::Value> {
    let config_path = src_path.join("config.toml");
    let config_contents = fs::read_to_string(config_path).unwrap();
    return config_contents.parse::<Table>().unwrap();
}

pub fn create_tera_config(src_path: &PathBuf) -> tera::Context {
    let config = load_config(src_path);
    let mut context = tera::Context::new();

    let vars = config.get("vars");

    if let Some(vars) = vars {
        for (key, val) in vars.as_table().unwrap() {
            context.insert(key, val);
        }
    }

    return context;
}
