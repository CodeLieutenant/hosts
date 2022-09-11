use std::{fmt::Display, ops::RangeBounds};

use smallvec::SmallVec;

use crate::tokens::Tokens;

#[derive(Debug, Eq, PartialEq)]
pub enum CstNode {
    Host(String),
    IP(std::net::IpAddr),
    Comment(String),
    Space,
    Tab,
    CarriageReturn,
    NewLine,
}

impl From<Tokens> for CstNode {
    fn from(token: Tokens) -> Self {
        match token {
            Tokens::Comment(c) => CstNode::Comment(c),
            Tokens::Space => CstNode::Space,
            Tokens::Tab => CstNode::Tab,
            Tokens::CarriageReturn => CstNode::CarriageReturn,
            Tokens::NewLine => CstNode::NewLine,
            _ => unreachable!("This should not happen"),
        }
    }
}

impl Display for CstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CstNode::Host(host) => write!(f, "{}", host),
            CstNode::IP(ip) => write!(f, "{}", ip),
            CstNode::Comment(comment) => write!(f, "#{}", comment),
            CstNode::Space => write!(f, " "),
            CstNode::Tab => write!(f, "\t"),
            CstNode::CarriageReturn => write!(f, "\r"),
            CstNode::NewLine => writeln!(f),
        }
    }
}

#[derive(Debug)]
pub struct Cst<const LENGTH: usize> {
    pub nodes: SmallVec<[CstNode; LENGTH]>,
}

impl<const LENGTH: usize> Cst<LENGTH> {
    pub fn remove_nodes<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        self.nodes.drain(range);
    }

    pub fn add_nodes<T, const OTHER: usize>(&mut self, nodes: T)
    where
        T: Into<SmallVec<[CstNode; OTHER]>>,
    {
        self.nodes.extend(nodes.into().into_iter());
    }

    pub fn add_node<T>(&mut self, node: T)
    where
        T: Into<CstNode>,
    {
        self.nodes.push(node.into());
    }
}

impl<const LENGTH: usize> ToString for Cst<LENGTH> {
    fn to_string(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use smallvec::smallvec_inline;

    use super::*;

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

        let cst = smallvec_inline![
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

        assert_eq!(expected, Cst { nodes: cst }.to_string());
    }
}
