use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq)]
pub enum Tokens {
    HostOrIp(String),
    Comment(String),
    Space,
    Tab,
    CarriageReturn,
    NewLine,
}

impl Debug for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::HostOrIp(host_or_ip) => write!(f, "HostOrIp({})", host_or_ip),
            Tokens::Comment(comment) => write!(f, "Comment({})", comment),
            Tokens::Space => write!(f, "Space( )"),
            Tokens::Tab => write!(f, "Tab(\t)"),
            Tokens::CarriageReturn => write!(f, "CarriageReturn(\r)"),
            Tokens::NewLine => write!(f, "NewLine(\\n)"),
        }
    }
}


impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::HostOrIp(host_or_ip) => write!(f, "{}", host_or_ip),
            Tokens::Comment(comment) => write!(f, "#{}", comment),
            Tokens::Space => write!(f, " "),
            Tokens::Tab => write!(f, "\t"),
            Tokens::CarriageReturn => write!(f, "\r"),
            Tokens::NewLine => write!(f, ""),
        }
    }
}