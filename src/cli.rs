use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Generate {
        src_path: PathBuf,
        output_path: PathBuf       
    }
}

pub fn parse_and_run() {
    let args = Cli::parse(); 

    match args.cmd {
        Commands::Generate { src_path, output_path } => println!("Yoink")
    }
}