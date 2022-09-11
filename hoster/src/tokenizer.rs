use std::{
    io::{ErrorKind, Read},
    str::from_utf8,
};

use thiserror::Error as ThisError;

use crate::tokens::Tokens;

const READ_BUFFER_SIZE: usize = 32 * 1024;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}

#[derive(Debug)]
pub struct Tokenizer<T> {
    input: T,
    tokens: Vec<Tokens>,
}

#[inline]
const fn end_of_host_or_ip(c: &char) -> bool {
    *c != ' ' && *c != '\t' && *c != '\n' && *c != '\r'
}

#[inline]
const fn end_of_comment(c: &char) -> bool {
    *c != '\n' && *c != '\r'
}

type CheckEnding = fn(&char) -> bool;

fn get_data_and_separator_fn(token: &mut Tokens) -> Option<(&mut String, CheckEnding)> {
    match token {
        Tokens::HostOrIp(ref mut data) => Some((data, end_of_host_or_ip)),
        Tokens::Comment(ref mut data) => Some((data, end_of_comment)),
        _ => None,
    }
}

#[inline]
fn find_separator<F>(separator: F, slice: &str, start: usize) -> &str
where
    F: Fn(&char) -> bool,
{
    let c = slice[start..].chars().position(|c| !separator(&c));

    match c {
        Some(c) => &slice[start..start + c],
        None => &slice[start..],
    }
}

#[inline]
fn check_last_token(tokens: &mut [Tokens], slice: &str, start: usize) -> usize {
    if let Some(token) = tokens.last_mut() {
        if let Some((data, separator)) = get_data_and_separator_fn(token) {
            let s = find_separator(separator, slice, start);
            data.push_str(s);

            start + s.len()
        } else {
            0
        }
    } else {
        0
    }
}

impl<T> Tokenizer<T> {
    #[inline]
    pub fn get_tokens(self) -> Vec<Tokens> {
        self.tokens
    }

    #[inline]
    pub(crate) fn parse_slice(&mut self, slice: &[u8]) -> Result<(), Error> {
        let code_points = from_utf8(slice)?;

        let mut advance = check_last_token(&mut self.tokens, code_points, 0);

        loop {
            if advance >= code_points.len() {
                break;
            }

            match code_points[advance..].chars().next() {
                Some('#') => {
                    // +1 to skip the '#' character
                    let comment = code_points[advance + 1..]
                        .chars()
                        .take_while(end_of_comment)
                        .collect::<String>();

                    advance += comment.len() + 1;
                    self.tokens.push(Tokens::Comment(comment));
                }
                Some('\t') => {
                    advance += 1;
                    self.tokens.push(Tokens::Tab);
                }
                Some(' ') => {
                    self.tokens.push(Tokens::Space);
                    advance += 1;
                }
                Some('\n') => {
                    self.tokens.push(Tokens::NewLine);
                    advance += 1;
                }
                Some('\r') => {
                    self.tokens.push(Tokens::CarriageReturn);
                    advance += 1;
                }
                Some(_) => {
                    let host_or_ip = code_points[advance..]
                        .chars()
                        .take_while(end_of_host_or_ip)
                        .collect::<String>();

                    advance += host_or_ip.len();
                    self.tokens.push(Tokens::HostOrIp(host_or_ip));
                }
                None => break,
            }
        }

        Ok(())
    }
}

fn check_bom_bytes_utf8(buffer: &[u8]) -> bool {
    buffer.len() >= 3 && buffer[0] == 0xEF && buffer[1] == 0xBB && buffer[2] == 0xBF
}

fn check_bom_bytes_utf16(buffer: &[u8]) -> bool {
    buffer.len() >= 2 && buffer[0] == 0xFE && buffer[1] == 0xFF
}

impl<T: Read> Tokenizer<T> {
    pub fn new_with_reader(file: T) -> Tokenizer<T> {
        Tokenizer {
            input: file,
            tokens: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Self, Error> {
        let mut read_buffer = [0u8; READ_BUFFER_SIZE];
        let mut buff: &[u8];
        loop {
            let result = self.input.read(&mut read_buffer[..]);

            match result {
                Ok(0) => break,
                Ok(n) => {
                    buff = if check_bom_bytes_utf8(&read_buffer) {
                        &read_buffer[3..n]
                    } else if check_bom_bytes_utf16(&read_buffer) {
                        &read_buffer[2..n]
                    } else {
                        &read_buffer[..n]
                    };

                    self.parse_slice(buff)?;
                }
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e.into()),
                },
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_last_token_empty_tokens() {
        let mut tokens = vec![];
        let slice = "";

        let position = check_last_token(&mut tokens, slice, 0);

        assert_eq!(position, 0);
    }

    #[test]
    fn test_check_last_token_not_comment_or_host_or_ip() {
        let mut tokens = vec![Tokens::NewLine];
        let slice = "";

        let position = check_last_token(&mut tokens, slice, 0);

        assert_eq!(position, 0);
    }

    #[test]
    fn test_check_last_token_with_start() {
        let mut tokens = vec![Tokens::HostOrIp("".to_string())];
        let slice = "test\n";

        let position = check_last_token(&mut tokens, slice, 1);

        assert_eq!(position, 4);
    }

    #[test]
    fn test_check_last_token() {
        let mut tokens = vec![Tokens::HostOrIp("127.0.0.1".to_string())];
        let slice = " localhost";

        let position = check_last_token(&mut tokens, slice, 0);

        assert_eq!(position, 0);

        let mut tokens = vec![Tokens::Comment("Hello World".to_string())];
        let slice = " from localhost\n";

        let position = check_last_token(&mut tokens, slice, 0);

        assert_eq!(position, slice.len() - 1);

        let mut tokens = vec![Tokens::HostOrIp("127.0.0.1".to_string())];
        let slice = "localhost \n";

        let position = check_last_token(&mut tokens, slice, 0);

        assert_eq!(position, slice.len() - 2);
    }

    #[test]
    fn test_find_separator() {
        let slice = "hello world";
        let separator = |c: &char| *c != ' ';

        assert_eq!(find_separator(separator, slice, 0), "hello");
        assert_eq!(find_separator(separator, slice, 6), "world");
        assert_eq!(find_separator(separator, "world", 0), "world");
    }

    #[test]
    fn it_tokenizes_multiple_hosts_on_the_same_line() {
        let data = "\
127.0.0.1\tlocalhost
127.0.1.1\thp

# The following lines are desirable for IPv6 capable hosts
::1\tip6-localhost ip6-loopback
fe00::0 ip6-localnet
";

        let tokenizer = Tokenizer::new_with_reader(data.as_bytes());
        let tokens = tokenizer.parse();

        assert!(tokens.is_ok());

        assert_eq!(
            vec![
                Tokens::HostOrIp("127.0.0.1".to_string()),
                Tokens::Tab,
                Tokens::HostOrIp("localhost".to_string()),
                Tokens::NewLine,
                Tokens::HostOrIp("127.0.1.1".to_string()),
                Tokens::Tab,
                Tokens::HostOrIp("hp".to_string()),
                Tokens::NewLine,
                Tokens::NewLine,
                Tokens::Comment(
                    " The following lines are desirable for IPv6 capable hosts".to_string()
                ),
                Tokens::NewLine,
                Tokens::HostOrIp("::1".to_string()),
                Tokens::Tab,
                Tokens::HostOrIp("ip6-localhost".to_string()),
                Tokens::Space,
                Tokens::HostOrIp("ip6-loopback".to_string()),
                Tokens::NewLine,
                Tokens::HostOrIp("fe00::0".to_string()),
                Tokens::Space,
                Tokens::HostOrIp("ip6-localnet".to_string()),
                Tokens::NewLine,
            ],
            tokens.unwrap().get_tokens()
        );
    }

    #[test]
    fn it_parses_the_buffer() {
        let str = "\
# localhost name resolution is handled within DNS itself.
# Added by Docker Desktop
192.168.0.17\thost.docker.internal
192.168.0.17 gateway.docker.internal
# To allow the same kube context to work on the host and the container:
\t127.0.0.1\tkubernetes.docker.internal


# End of section
";
        let tokenizer = Tokenizer::new_with_reader(str.as_bytes());
        let tokens = tokenizer.parse();

        assert!(tokens.is_ok());

        assert_eq!(
            vec![
                Tokens::Comment(
                    " localhost name resolution is handled within DNS itself.".to_string()
                ),
                Tokens::NewLine,
                Tokens::Comment(" Added by Docker Desktop".to_string()),
                Tokens::NewLine,
                Tokens::HostOrIp("192.168.0.17".to_string()),
                Tokens::Tab,
                Tokens::HostOrIp("host.docker.internal".to_string()),
                Tokens::NewLine,
                Tokens::HostOrIp("192.168.0.17".to_string()),
                Tokens::Space,
                Tokens::HostOrIp("gateway.docker.internal".to_string()),
                Tokens::NewLine,
                Tokens::Comment(
                    " To allow the same kube context to work on the host and the container:"
                        .to_string()
                ),
                Tokens::NewLine,
                Tokens::Tab,
                Tokens::HostOrIp("127.0.0.1".to_string()),
                Tokens::Tab,
                Tokens::HostOrIp("kubernetes.docker.internal".to_string()),
                Tokens::NewLine,
                Tokens::NewLine,
                Tokens::NewLine,
                Tokens::Comment(" End of section".to_string()),
                Tokens::NewLine,
            ],
            tokens.unwrap().get_tokens()
        );
    }
}
