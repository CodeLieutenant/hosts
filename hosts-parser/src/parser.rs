use std::{fmt::Display, net::IpAddr};

use thiserror::Error as ThisError;

use crate::lookahead::LookaheadIter;
use crate::tokens::Tokens;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("unexpected token {0}")]
    UnexpectedToken(CST),

    #[error("Invalid ip {0}")]
    InvalidIp(String),

    #[error("Expecting token: {0}")]
    ExpectingToken(CST),
}

#[derive(Debug, Eq, PartialEq)]
pub enum CST {
    Host(String),
    IP(std::net::IpAddr),
    Comment(String),
    Space,
    Tab,
    CarriageReturn,
    NewLine,
}

impl From<Tokens> for CST {
    fn from(token: Tokens) -> Self {
        match token {
            Tokens::Comment(c) => CST::Comment(c),
            Tokens::Space => CST::Space,
            Tokens::Tab => CST::Tab,
            Tokens::CarriageReturn => CST::CarriageReturn,
            Tokens::NewLine => CST::NewLine,
            _ => unreachable!("This should not happen"),
        }
    }
}

impl Display for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CST::Host(host) => write!(f, "{}", host),
            CST::IP(ip) => write!(f, "{}", ip),
            CST::Comment(comment) => write!(f, "#{}", comment),
            CST::Space => write!(f, " "),
            CST::Tab => write!(f, "\t"),
            CST::CarriageReturn => write!(f, "\r"),
            CST::NewLine => write!(f, "\n"),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub(crate) cst: Vec<CST>,
}

impl ToString for Parser {
    fn to_string(&self) -> String {
        self.cst
            .iter()
            .map(|cst| cst.to_string())
            .collect::<String>()
    }
}

impl Parser {
    pub fn parse(tokens: Vec<Tokens>) -> Result<Self, Error> {
        let len = tokens.len();
        let mut cst = Vec::with_capacity(len);
        let lookahead = LookaheadIter {
            iter: tokens.into_iter(),
        };

        let mut is_ip_parsed = false;

        for (token, next) in lookahead {
            match token {
                Tokens::HostOrIp(ip) => {
                    if !is_ip_parsed {
                        let ip = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;
                        cst.push(CST::IP(ip));
                        is_ip_parsed = true;
                    } else {
                        cst.push(CST::Host(ip));
                        is_ip_parsed = false;
                    }

                    if let Some(next) = next {
                        cst.push(CST::from(next));
                    }
                }
                Tokens::Comment(comment) => {
                    if let Some(Tokens::NewLine | Tokens::CarriageReturn) | None = next {
                        cst.push(CST::Comment(comment));
                    } else {
                        return Err(Error::UnexpectedToken(CST::Comment(comment)));
                    }

                    if let Some(next) = next {
                        cst.push(CST::from(next));
                    }
                }
                Tokens::Space | Tokens::Tab => {
                    cst.push(CST::from(token));

                    if let Some(Tokens::HostOrIp(host)) = next {
                        if !is_ip_parsed {
                            let ip = host.parse::<IpAddr>().map_err(|_| Error::InvalidIp(host))?;
                            cst.push(CST::IP(ip));
                            is_ip_parsed = true;
                        } else {
                            cst.push(CST::Host(host));
                            is_ip_parsed = false;
                        }
                    }
                }
                Tokens::CarriageReturn => {
                    cst.push(CST::CarriageReturn);

                    match next {
                        Some(Tokens::NewLine) => {
                            cst.push(CST::NewLine);
                        }
                        Some(t) => return Err(Error::UnexpectedToken(CST::from(t))),
                        None => return Err(Error::UnexpectedToken(CST::NewLine)),
                    };
                }
                Tokens::NewLine => {
                    cst.push(CST::from(token));

                    match next {
                        Some(Tokens::HostOrIp(ip)) => {
                            let ip = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;
                            cst.push(CST::IP(ip));
                            is_ip_parsed = true;
                        }
                        Some(next) => cst.push(CST::from(next)),
                        None => {}
                    }
                }
            }
        }

        Ok(Self { cst })
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_parse_tokens() {
        let tokens = vec![
            Tokens::HostOrIp("127.0.0.1".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("localhost".to_string()),
        ];
        let parser = Parser::parse(tokens);

        assert!(parser.is_ok());
        assert_eq!(
            vec![
                CST::IP(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                CST::Space,
                CST::Host("localhost".to_string())
            ],
            parser.unwrap().cst
        );

        let tokens = vec![
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
            Tokens::Comment(
                " To allow the same kube context to work on the host and the container:"
                    .to_string(),
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
        ];

        let parser = Parser::parse(tokens);
        assert!(parser.is_ok());
        assert_eq!(
            vec![
                CST::Comment(
                    " localhost name resolution is handled within DNS itself.".to_string()
                ),
                CST::NewLine,
                CST::Comment(" Added by Docker Desktop".to_string()),
                CST::NewLine,
                CST::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
                CST::Tab,
                CST::Host("host.docker.internal".to_string()),
                CST::NewLine,
                CST::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
                CST::Space,
                CST::Host("gateway.docker.internal".to_string()),
                CST::NewLine,
                CST::Comment(
                    " To allow the same kube context to work on the host and the container:"
                        .to_string()
                ),
                CST::NewLine,
                CST::Tab,
                CST::IP("127.0.0.1".parse::<IpAddr>().unwrap()),
                CST::Tab,
                CST::Host("kubernetes.docker.internal".to_string()),
                CST::NewLine,
                CST::NewLine,
                CST::NewLine,
                CST::Comment(" End of section".to_string()),
                CST::NewLine,
            ],
            parser.unwrap().cst
        );
    }

    #[test]
    fn test_to_string() {
        let expected = "\
# localhost name resolution is handled within DNS itself.
# Added by Docker Desktop
192.168.0.17\thost.docker.internal
192.168.0.17 gateway.docker.internal
# To allow the same kube context to work on the host and the container:
\t127.0.0.1\tkubernetes.docker.internal


# End of section
";

        let cst = vec![
            CST::Comment(" localhost name resolution is handled within DNS itself.".to_string()),
            CST::NewLine,
            CST::Comment(" Added by Docker Desktop".to_string()),
            CST::NewLine,
            CST::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
            CST::Tab,
            CST::Host("host.docker.internal".to_string()),
            CST::NewLine,
            CST::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
            CST::Space,
            CST::Host("gateway.docker.internal".to_string()),
            CST::NewLine,
            CST::Comment(
                " To allow the same kube context to work on the host and the container:"
                    .to_string(),
            ),
            CST::NewLine,
            CST::Tab,
            CST::IP("127.0.0.1".parse::<IpAddr>().unwrap()),
            CST::Tab,
            CST::Host("kubernetes.docker.internal".to_string()),
            CST::NewLine,
            CST::NewLine,
            CST::NewLine,
            CST::Comment(" End of section".to_string()),
            CST::NewLine,
        ];

        let parser = Parser { cst };

        assert_eq!(expected, parser.to_string());
    }
}
