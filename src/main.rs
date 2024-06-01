use std::env;
use env_logger;

mod cli;
mod generate;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    cli::parse_and_run();
}
