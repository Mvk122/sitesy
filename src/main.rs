use std::env;
use env_logger;

mod cli;
mod generate;
mod load_config;
mod tera_funcs;
mod new;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    cli::parse_and_run();
}
