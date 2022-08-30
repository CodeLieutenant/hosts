use std::net::IpAddr;
use std::ops::RangeBounds;

use smallvec::SmallVec;
use thiserror::Error as ThisError;

use crate::cst::CstNode;
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
    pub(crate) cst: Vec<CstNode>,
    pub(crate) visitor: Option<V>,
}

impl<V> ToString for Parser<V> {
    fn to_string(&self) -> String {
        self.cst
            .iter()
            .map(|cst| cst.to_string())
            .collect::<String>()
    }
}

fn parse_ip(ip: String) -> Result<IpAddr, Error> {
    let addr = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;

    Ok(addr)
}

impl<V: CstVisitor> Parser<V> {
    pub fn visit(&mut self) {
        let visitor = match self.visitor.as_mut() {
            Some(v) => v,
            None => return,
        };

        for cst in self.cst.iter() {
            visitor.visit(cst);
        }
    }
}

impl<V> Default for Parser<V> {
    fn default() -> Self {
        Self {
            cst: Vec::new(),
            visitor: None,
        }
    }
}

impl<V> Parser<V> {
    pub fn with_visitor(self, visitor: V) -> Self
    where
        V: CstVisitor,
    {
        Self {
            cst: self.cst,
            visitor: Some(visitor),
        }
    }

    pub fn remove_nodes<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.cst.drain(range);
    }

    pub fn add_nodes<T, const LENGTH: usize>(&mut self, nodes: T)
    where
        T: Into<SmallVec<[CstNode; LENGTH]>>,
    {
        self.cst.extend(nodes.into().into_iter());
    }

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
                        let ip = parse_ip(ip)?;
                        cst.push(CstNode::IP(ip));
                        is_ip_parsed = true;
                    } else {
                        cst.push(CstNode::Host(ip));
                    }

                    if let Some(next) = next {
                        if next == Tokens::NewLine {
                            is_ip_parsed = false;
                        }

                        cst.push(CstNode::from(next));
                    }
                }
                Tokens::Comment(comment) => {
                    if let Some(Tokens::NewLine | Tokens::CarriageReturn) | None = next {
                        cst.push(CstNode::Comment(comment));
                    } else {
                        return Err(Error::UnexpectedToken(CstNode::Comment(comment)));
                    }

                    if let Some(next) = next {
                        cst.push(CstNode::from(next));
                    }
                }
                Tokens::Space | Tokens::Tab => {
                    cst.push(CstNode::from(token));

                    if let Some(Tokens::HostOrIp(host)) = next {
                        if !is_ip_parsed {
                            let ip = host.parse::<IpAddr>().map_err(|_| Error::InvalidIp(host))?;
                            cst.push(CstNode::IP(ip));
                            is_ip_parsed = true;
                        } else {
                            cst.push(CstNode::Host(host));
                        }
                    }
                }
                Tokens::CarriageReturn => {
                    cst.push(CstNode::CarriageReturn);

                    match next {
                        Some(Tokens::NewLine) => {
                            cst.push(CstNode::NewLine);
                        }
                        Some(t) => return Err(Error::UnexpectedToken(CstNode::from(t))),
                        None => return Err(Error::UnexpectedToken(CstNode::NewLine)),
                    };
                }
                Tokens::NewLine => {
                    cst.push(CstNode::from(token));

                    match next {
                        Some(Tokens::HostOrIp(ip)) => {
                            let ip = ip.parse::<IpAddr>().map_err(|_| Error::InvalidIp(ip))?;
                            cst.push(CstNode::IP(ip));
                            is_ip_parsed = true;
                        }
                        Some(next) => cst.push(CstNode::from(next)),
                        None => {}
                    }
                }
            }
        }

        Ok(Self { cst, visitor: None })
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
        let parser = Parser::<()>::parse(tokens);

        assert!(parser.is_ok());
        assert_eq!(
            vec![
                CstNode::IP(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                CstNode::Space,
                CstNode::Host("localhost".to_string())
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

        let parser = Parser::<()>::parse(tokens);
        assert!(parser.is_ok());
        assert_eq!(
            vec![
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
            parser.unwrap().cst
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

        let parser = Parser::<()>::parse(tokens);
        assert!(parser.is_ok());

        assert_eq!(
            vec![
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
            CstNode::Comment(
                " localhost name resolution is handled within DNS itself.".to_string(),
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
                    .to_string(),
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
        ];

        let parser = Parser {
            cst,
            visitor: Option::<()>::None,
        };

        assert_eq!(expected, parser.to_string());
    }
}
