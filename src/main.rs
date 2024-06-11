use env_logger;
use std::env;

mod cli;
mod css;
mod generate;
mod load_config;
mod new;
mod template_matching;
mod tera_funcs;
mod server;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    cli::parse_and_run();
}
