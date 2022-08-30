use std::{error::Error, fs::File, path::PathBuf, str::FromStr};

use hosts_parser::parser::Parser;
use hosts_parser::tokenizer::Tokenizer;

const LINUX_HOSTS_PATH: &str = "/etc/hosts";
const MACOS_HOSTS_PATH: &str = "/etc/hosts";
const WINDOWS_HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[inline]
const fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "linux") {
        LINUX_HOSTS_PATH
    } else if cfg!(target_os = "windows") {
        WINDOWS_HOSTS_PATH
    } else if cfg!(target_os = "macos") {
        MACOS_HOSTS_PATH
    } else {
        panic!("Unsupported operating system");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let hosts_file = File::options()
        .read(true)
        .write(false)
        .truncate(false)
        .append(false)
        .open(PathBuf::from_str(get_hosts_path())?)?;

    let tokens = Tokenizer::new_with_reader(&hosts_file)
        .parse()?
        .get_tokens();

    let parser = Parser::parse(tokens)?;

    // println!("{:?}", tokens);
    println!("{}", parser.to_string());

    Ok(())
}
