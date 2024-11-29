mod init;
mod template;

use clap::ColorChoice;
use clap::{Parser, Subcommand};
use crate::template::TEMPLEX_DIRECTORY;

/// Templex is a tool to manage LaTeX projects
#[derive(Parser)]
#[command(version, about, long_about = None, color = ColorChoice::Auto)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a LaTeX project
    Init {
        /// lists test values
        #[arg()]
        template: String,
    },
    /// Outputs the directory and a list of templates.
    List
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { template } => {
            init::cli_init(template::resolve_template(template)).map_err(|err| err.into())
        },
        Commands::List => {
            let mut x = TEMPLEX_DIRECTORY.clone();
            x.push("templates/");
            println!("Reading templates from {}", x.display());
            for i in template::get_templates() {
                println!("--> {i}")
            }
            Ok(())
        }
    }
}