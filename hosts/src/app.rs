use std::io::{Cursor, Seek, SeekFrom, Write};
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::commands::add::execute as add_command;
use crate::commands::remove::execute as remove_command;

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
where
    P: Into<PathBuf>,
{
    let app = App::parse();

    let mut file_options = File::options();

    match app.commands {
        Commands::Add { host, ip, comment } => {
            add_command(
                file_options.append(true).open(path.into())?,
                ip,
                host,
                comment,
            )?;
        }
        Commands::Remove { host } => {
            let mut hosts = file_options.open(path.into())?;
            let mut data = Vec::with_capacity(2048);

            remove_command(&mut hosts, Cursor::new(&mut data), host)?;

            hosts.seek(SeekFrom::Start(0))?;
            hosts.set_len(data.len() as u64)?;
            let _n = hosts.write(&data)?;
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
