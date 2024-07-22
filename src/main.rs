use env_logger;
use std::env;

mod cli;
mod css;
mod frontmatter;
mod generate;
mod iterator;
mod load_config;
mod new;
mod server;
mod template_matching;
mod tera_funcs;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    cli::parse_and_run();
}
