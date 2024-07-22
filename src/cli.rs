use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{generate::generate, new::create_new_ssg_project, server::serve};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[clap(about = "Run static site generation")]
    Generate {
        src_path: PathBuf,
        output_path: PathBuf,
    },
    #[clap(
        about = "Runs the sitesy server which runs automatic recompilation when your sites files change"
    )]
    Server {
        src_path: PathBuf,
        output_path: PathBuf,
    },
    #[clap(about = "Create a new sitesy project")]
    New { output_path: PathBuf },
}

pub fn parse_and_run() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Generate {
            src_path,
            output_path,
        } => match generate(&src_path, &output_path) {
            Ok(ok) => {
                println!("Success: {}", ok);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        },
        Commands::Server {
            src_path,
            output_path,
        } => serve(src_path, output_path),
        Commands::New { output_path } => {
            create_new_ssg_project(output_path);
        }
    }
}
