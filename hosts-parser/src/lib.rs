use std::{
    io::{ErrorKind, Read},
    str::from_utf8,
};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}

#[derive(Debug, Eq, PartialEq)]
enum Tokens {
    HostOrIp(String),
    Comment(String),
    Space,
    Tab,
    CarriageReturn,
    NewLine,
}

#[derive(Debug)]
struct Tokenizer<T> {
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

fn get_data_and_separator_fn(token: &mut Tokens) -> Option<(&mut String, fn(&char) -> bool)> {
    match token {
        Tokens::HostOrIp(ref mut data) => Some((data, end_of_host_or_ip)),
        Tokens::Comment(ref mut data) => Some((data, end_of_comment)),
        _ => None,
    }
}

#[inline]
fn find_separator<F>(separator: F, slice: &str, start: usize) -> &str
    where F: Fn(&char) -> bool {
    let c = slice[start..]
        .chars()
        .position(|c| !separator(&c));

    match c {
        Some(c) => &slice[start..start + c],
        None => slice
    }
}

#[inline]
fn check_last_token(tokens: &mut Vec<Tokens>, slice: &str, start: usize) -> usize {
    if let Some(token) = tokens.last_mut() {
        if let Some((data, separator)) = get_data_and_separator_fn(token) {
            let s = find_separator(separator, slice, start);
            data.push_str(s);

            return start + s.len();
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

impl<T: Read> Tokenizer<T> {
    pub fn new_with_reader(file: T) -> Tokenizer<T> {
        Tokenizer {
            input: file,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        let mut buf = [0u8; 4096];

        loop {
            let result = self.input.read(&mut buf[..]);

            match result {
                Ok(0) => break,
                Ok(n) => {
                    self.parse_slice(&buf[..n])?;
                }
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e.into()),
                },
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut tokenizer = Tokenizer::new_with_reader(str.as_bytes());
        let tokens = tokenizer.parse();

        assert!(tokens.is_ok());

        assert_eq!(vec![
            Tokens::Comment(" localhost name resolution is handled within DNS itself.".to_string()),
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
            Tokens::Comment(" To allow the same kube context to work on the host and the container:".to_string()),
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
        ], tokenizer.get_tokens());
    }
}