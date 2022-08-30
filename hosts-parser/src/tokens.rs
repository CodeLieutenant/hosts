use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug)]
pub enum Tokens {
    HostOrIp(String),
    Comment(String),
    Space,
    Tab,
    CarriageReturn,
    NewLine,
}
