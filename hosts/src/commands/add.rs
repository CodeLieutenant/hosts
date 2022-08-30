use std::io::Write;

use hosts_parser::{cst::CstNode, parser::Parser};
use smallvec::{smallvec, SmallVec};

pub(crate) fn execute<W>(
    mut writer: W,
    ip: String,
    host: String,
    comment: Option<String>,
) -> Result<(), Box<dyn std::error::Error>>
where
    W: Write,
{
    let mut parser = Parser::<()>::default();

    let cst: SmallVec<[CstNode; 6]> = match comment {
        Some(comment) => smallvec![
            CstNode::Comment(comment),
            CstNode::NewLine,
            CstNode::IP(ip.parse()?),
            CstNode::Space,
            CstNode::Host(host),
            CstNode::NewLine
        ],
        None => smallvec![
            CstNode::IP(ip.parse()?),
            CstNode::Space,
            CstNode::Host(host),
            CstNode::NewLine
        ],
    };

    parser.add_nodes(cst);

    write!(writer, "{}", parser.to_string())?;

    Ok(())
}
