use std::fmt::Display;

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
