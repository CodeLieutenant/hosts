use std::net::IpAddr;

use thiserror::Error as ThisError;

use crate::cst::{Cst, CstNode};
use crate::lookahead::LookaheadIter;
use crate::tokens::Tokens;
use crate::visitor::CstVisitor;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("unexpected token {0}")]
    UnexpectedToken(CstNode),

    #[error("Invalid ip {0}")]
    InvalidIp(String),

    #[error("Expecting token: {0}")]
    ExpectingToken(CstNode),
}

#[derive(Debug)]
pub struct Parser<V> {
    pub(crate) visitor: Option<V>,
}

#[derive(Debug)]
pub struct ParserBuilder<V> {
    pub(crate) visitor: Option<V>,
}

impl<V> Default for ParserBuilder<V> {
    fn default() -> Self {
        Self { visitor: None }
    }
}

impl<V> ParserBuilder<V> {
    pub fn visitor(mut self, visitor: V) -> Self {
        self.visitor = Some(visitor);
        self
    }

    pub fn build(self) -> Parser<V> {
        Parser {
            visitor: self.visitor,
        }
    }
}

fn parse_ip(ip: String) -> Result<IpAddr, Error> {
    let addr = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;

    Ok(addr)
}

impl<V: CstVisitor> Parser<V> {
    pub fn visit<const LENGTH: usize>(&mut self, cst: &Cst<LENGTH>) {
        let visitor = match self.visitor.as_mut() {
            Some(v) => v,
            None => return,
        };

        for (idx, node) in cst.nodes.iter().enumerate() {
            if visitor.visit(idx, node).is_none() {
                break;
            }
        }
    }

    pub fn get_visitor(&self) -> Option<&V> {
        self.visitor.as_ref()
    }
}

impl Default for Parser<()> {
    fn default() -> Self {
        Self { visitor: None }
    }
}

impl<V> Parser<V> {
    pub fn builder() -> ParserBuilder<V> {
        ParserBuilder::<V>::default()
    }

    pub fn parse<const LENGTH: usize>(&self, tokens: Vec<Tokens>) -> Result<Cst<LENGTH>, Error> {
        let mut cst = Cst::<LENGTH> {
            nodes: smallvec::smallvec![],
        };
        let lookahead = LookaheadIter {
            iter: tokens.into_iter(),
        };

        let mut is_ip_parsed = false;

        for (token, next) in lookahead {
            match token {
                Tokens::HostOrIp(ip) => {
                    if !is_ip_parsed {
                        let ip = parse_ip(ip)?;
                        cst.add_node(CstNode::IP(ip));
                        is_ip_parsed = true;
                    } else {
                        cst.add_node(CstNode::Host(ip));
                    }

                    if let Some(next) = next {
                        if next == Tokens::NewLine {
                            is_ip_parsed = false;
                        }

                        cst.add_node(CstNode::from(next));
                    }
                }
                Tokens::Comment(comment) => {
                    if let Some(Tokens::NewLine | Tokens::CarriageReturn) | None = next {
                        cst.add_node(CstNode::Comment(comment));
                    } else {
                        return Err(Error::UnexpectedToken(CstNode::Comment(comment)));
                    }

                    if let Some(next) = next {
                        cst.add_node(CstNode::from(next));
                    }
                }
                Tokens::Space | Tokens::Tab => {
                    cst.add_node(CstNode::from(token));

                    if let Some(Tokens::HostOrIp(host)) = next {
                        if !is_ip_parsed {
                            let ip = host.parse::<IpAddr>().map_err(|_| Error::InvalidIp(host))?;
                            cst.add_node(CstNode::IP(ip));
                            is_ip_parsed = true;
                        } else {
                            cst.add_node(CstNode::Host(host));
                        }
                    }
                }
                Tokens::CarriageReturn => {
                    cst.add_node(CstNode::CarriageReturn);

                    match next {
                        Some(Tokens::NewLine) => {
                            cst.add_node(CstNode::NewLine);
                            is_ip_parsed = false;
                        }
                        Some(t) => return Err(Error::UnexpectedToken(CstNode::from(t))),
                        None => return Err(Error::UnexpectedToken(CstNode::NewLine)),
                    };
                }
                Tokens::NewLine => {
                    cst.add_node(CstNode::from(token));

                    match next {
                        Some(Tokens::HostOrIp(ip)) => {
                            let ip = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;
                            cst.add_node(CstNode::IP(ip));
                            is_ip_parsed = true;
                        }
                        Some(next) => cst.add_node(CstNode::from(next)),
                        None => {}
                    }
                }
            }
        }

        Ok(cst)
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use smallvec::smallvec_inline;

    use super::*;

    #[test]
    fn test_parse_tokens() {
        let tokens = vec![
            Tokens::HostOrIp("127.0.0.1".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("localhost".to_string()),
        ];

        let parser = Parser::default();
        let cst = parser.parse::<1>(tokens);
        assert!(cst.is_ok());
        assert_eq!(
            smallvec_inline![
                CstNode::IP(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                CstNode::Space,
                CstNode::Host("localhost".to_string())
            ],
            cst.unwrap().nodes
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

        let parser = Parser::default();

        let cst = parser.parse::<1>(tokens);
        assert!(cst.is_ok());
        assert_eq!(
            smallvec_inline![
                CstNode::Comment(
                    " localhost name resolution is handled within DNS itself.".to_string()
                ),
                CstNode::NewLine,
                CstNode::Comment(" Added by Docker Desktop".to_string()),
                CstNode::NewLine,
                CstNode::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("host.docker.internal".to_string()),
                CstNode::NewLine,
                CstNode::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
                CstNode::Space,
                CstNode::Host("gateway.docker.internal".to_string()),
                CstNode::NewLine,
                CstNode::Comment(
                    " To allow the same kube context to work on the host and the container:"
                        .to_string()
                ),
                CstNode::NewLine,
                CstNode::Tab,
                CstNode::IP("127.0.0.1".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("kubernetes.docker.internal".to_string()),
                CstNode::NewLine,
                CstNode::NewLine,
                CstNode::NewLine,
                CstNode::Comment(" End of section".to_string()),
                CstNode::NewLine,
            ],
            cst.unwrap().nodes
        );
    }

    #[test]
    fn test_with_carriage_return() {
        let tokens = vec![
            Tokens::Comment(" Added by Docker Desktop".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("192.168.0.17".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("host.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("192.168.0.17".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("gateway.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " To allow the same kube context to work on the host and the container:"
                    .to_string(),
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("127.0.0.1".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("kubernetes.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" End of section".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
        ];

        let parser = Parser::default();

        let cst = parser.parse::<1>(tokens);
        assert!(cst.is_ok());

        let cst = cst.unwrap();

        assert_eq!(
            smallvec_inline![
                CstNode::Comment(" Added by Docker Desktop".to_string()),
                CstNode::CarriageReturn,
                CstNode::NewLine,
                CstNode::IP("192.168.0.17".parse().unwrap()),
                CstNode::Space,
                CstNode::Host("host.docker.internal".to_string()),
                CstNode::CarriageReturn,
                CstNode::NewLine,
                CstNode::IP("192.168.0.17".parse().unwrap()),
                CstNode::Space,
                CstNode::Host("gateway.docker.internal".to_string()),
                CstNode::CarriageReturn,
                CstNode::NewLine,
                CstNode::Comment(
                    " To allow the same kube context to work on the host and the container:"
                        .to_string()
                ),
                CstNode::CarriageReturn,
                CstNode::NewLine,
                CstNode::IP("127.0.0.1".parse().unwrap()),
                CstNode::Space,
                CstNode::Host("kubernetes.docker.internal".to_string()),
                CstNode::CarriageReturn,
                CstNode::NewLine,
                CstNode::Comment(" End of section".to_string()),
                CstNode::CarriageReturn,
                CstNode::NewLine,
            ],
            cst.nodes
        );
    }

    #[test]
    fn test_multiple_hosts_on_the_same_line() {
        let tokens = vec![
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
                " The following lines are desirable for IPv6 capable hosts".to_string(),
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
        ];

        let parser = Parser::default();

        let cst = parser.parse::<1>(tokens);
        assert!(cst.is_ok());

        assert_eq!(
            smallvec_inline![
                CstNode::IP("127.0.0.1".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("localhost".to_string()),
                CstNode::NewLine,
                CstNode::IP("127.0.1.1".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("hp".to_string()),
                CstNode::NewLine,
                CstNode::NewLine,
                CstNode::Comment(
                    " The following lines are desirable for IPv6 capable hosts".to_string(),
                ),
                CstNode::NewLine,
                CstNode::IP("::1".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("ip6-localhost".to_string()),
                CstNode::Space,
                CstNode::Host("ip6-loopback".to_string()),
                CstNode::NewLine,
                CstNode::IP("fe00::0".parse::<IpAddr>().unwrap()),
                CstNode::Space,
                CstNode::Host("ip6-localnet".to_string()),
                CstNode::NewLine,
            ],
            cst.unwrap().nodes
        );
    }

    #[test]
    fn test_multiple_hosts_on_the_same_line_with_carriage_return() {
        let tokens = vec![
            Tokens::HostOrIp("192.168.0.17".to_string()),
            Tokens::Tab,
            Tokens::HostOrIp("host.docker.internal".to_string()),
            Tokens::Tab,
            Tokens::HostOrIp("another.com".to_string()),
            Tokens::Space,
            Tokens::Comment(" Comment".to_string()),
            Tokens::NewLine,
        ];

        let parser = Parser::default();

        let cst = parser.parse::<10>(tokens);
        assert!(cst.is_ok());

        let cst = cst.unwrap();

        assert_eq!(
            smallvec_inline![
                CstNode::IP("192.168.0.17".parse::<IpAddr>().unwrap()),
                CstNode::Tab,
                CstNode::Host("host.docker.internal".to_string()),
                CstNode::Tab,
                CstNode::Host("another.com".to_string()),
                CstNode::Space,
                CstNode::Comment(" Comment".to_string()),
                CstNode::NewLine,
            ],
            cst.nodes
        );
    }
}
