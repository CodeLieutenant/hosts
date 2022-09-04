use std::io::Write;

use hosts_parser::cst::{Cst, CstNode};
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

    let cst = Cst { nodes: cst };

    write!(writer, "{}", cst.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_add_command_without_comment() {
        let mut writer = Cursor::new(Vec::new());

        let result = execute(
            &mut writer,
            "127.0.0.1".to_string(),
            "localhost".to_string(),
            None,
        );

        assert!(result.is_ok());

        writer.set_position(0);
        let mut data = Vec::new();

        assert!(writer.read_to_end(&mut data).is_ok());
        assert_eq!(
            String::from_utf8(data).unwrap(),
            "127.0.0.1 localhost\n".to_string()
        );
    }

    #[test]
    fn test_add_command_with_comment() {
        let mut writer = Cursor::new(Vec::new());

        let result = execute(
            &mut writer,
            "127.0.0.1".to_string(),
            "localhost".to_string(),
            Some(" here is my comment".to_string()),
        );

        assert!(result.is_ok());

        writer.set_position(0);
        let mut data = Vec::new();

        assert!(writer.read_to_end(&mut data).is_ok());
        assert_eq!(
            String::from_utf8(data).unwrap(),
            "# here is my comment\n127.0.0.1 localhost\n".to_string()
        );
    }
}
