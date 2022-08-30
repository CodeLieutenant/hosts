use std::{error::Error, fs::File, path::PathBuf, str::FromStr};

use hosts_parser::tokenizer::Tokenizer;
use hosts_parser::parser::Parser;

const LINUX_HOSTS_PATH: &str = "/etc/hosts";
const WINDOWS_HOSTS_PATH: &str = "";

#[inline]
const fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "linux") {
        LINUX_HOSTS_PATH
    } else if cfg!(target_os = "windows") {
        WINDOWS_HOSTS_PATH
    } else {
        panic!("Unsupported operating system");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let hosts_file = File::options()
        .read(true)
        .write(false)
        .truncate(false)
        .open(PathBuf::from_str(get_hosts_path())?)?;

    let tokens = Tokenizer::new_with_reader(&hosts_file)
        .parse()?
        .get_tokens();

    let parser = Parser::parse(tokens)?;

    // println!("{:?}", tokens);
    println!("{}", parser.to_string());

    Ok(())
}
