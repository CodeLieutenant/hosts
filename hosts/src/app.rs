use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::commands::add::execute as add_command;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Add {
        #[clap(required = false, value_parser)]
        host: String,
        #[clap(required = false, value_parser)]
        ip: String,
        #[clap(required = false, value_parser)]
        comment: Option<String>,
    },
    Remove {
        #[clap(required = false, value_parser)]
        host: String,
    },
    List {
        #[clap(required = false, value_parser)]
        with_comments: bool,
    },
    Version,
}

#[derive(Debug, Parser)]
#[clap(name = "hosts",about = "Parses and modified OS Hosts file", long_about = None)]
pub struct App {
    #[clap(subcommand)]
    commands: Commands,
}

pub fn execute<P>(path: P) -> Result<(), Box<dyn std::error::Error>>
where P: Into<PathBuf>
{
    let app = App::parse();

    let mut file_options = File::options();
    file_options.truncate(false);

    match app.commands {
        Commands::Add { host, ip, comment } => {
            add_command(file_options.append(true).open(path.into())?, ip, host, comment)?;
        }
        Commands::Remove { host } => {
            println!("{}", host);
        }
        Commands::List { with_comments } => {
            println!("{}", with_comments);
        }
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
