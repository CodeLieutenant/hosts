use std::io::{Read, Write};

use hoster::{cst::CstNode, parser::Parser, tokenizer::Tokenizer, visitor::CstVisitor};

pub(crate) fn execute<R, W>(
    mut reader: R,
    mut writer: W,
    host: String,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: Read,
    W: Write,
{
    let tokens = Tokenizer::new_with_reader(&mut reader)
        .parse()?
        .get_tokens();

    let mut parser = Parser::builder()
        .visitor(Visitor::new(host.as_str()))
        .build();

    let mut cst = parser.parse::<1>(tokens)?;
    parser.visit(&cst);

    let visitor = parser.get_visitor().unwrap();

    if visitor.has_found() {
        cst.remove_nodes(visitor.get_start()..=visitor.get_end());
        writer.write_all(cst.to_string().as_bytes())?;
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct Visitor<'a> {
    start: usize,
    end: usize,
    hosts_on_line: u32,
    host_to_remove_pos: Option<usize>,
    host: &'a str,
}

impl<'a> Visitor<'a> {
    fn new(host: &'a str) -> Self {
        Self {
            start: 0,
            end: 0,
            host,
            hosts_on_line: 0,
            host_to_remove_pos: None,
        }
    }

    pub fn get_start(&self) -> usize {
        if self.hosts_on_line > 1 {
            self.host_to_remove_pos.unwrap()
        } else {
            self.start
        }
    }

    pub fn get_end(&self) -> usize {
        if self.hosts_on_line > 1 {
            self.host_to_remove_pos.unwrap()
        } else {
            self.end
        }
    }

    pub fn has_found(&self) -> bool {
        self.host_to_remove_pos.is_some()
    }
}

impl<'a> CstVisitor for Visitor<'a> {
    fn visit(&mut self, idx: usize, node: &CstNode) -> Option<()> {
        if let CstNode::NewLine = node {
            if self.host_to_remove_pos.is_some() {
                self.end = idx;
                return None;
            }

            self.start = idx + 1;
            self.hosts_on_line = 0;
        }

        if let CstNode::Host(host) = node {
            if host == self.host {
                self.host_to_remove_pos = Some(idx);
            }

            self.hosts_on_line += 1;
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_remove_command() {
        let mut reader = Cursor::new(Vec::from(
            "# here is my comment\n127.0.0.1 localhost\n127.0.0.1 other-domain.com\n".to_string(),
        ));
        let mut writer = Cursor::new(Vec::new());

        let result = execute(&mut reader, &mut writer, "localhost".to_string());

        assert!(result.is_ok());

        writer.set_position(0);

        let mut data = Vec::new();
        let result = writer.read_to_end(&mut data);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(data).unwrap(),
            "# here is my comment\n127.0.0.1 other-domain.com\n".to_string()
        );
    }

    #[test]
    fn test_remove_command_not_found() {
        let mut reader = Cursor::new(Vec::from(
            "# here is my comment\n127.0.0.1 localhost\n127.0.0.1 other-domain.com\n".to_string(),
        ));
        let mut writer = Cursor::new(Vec::new());

        let result = execute(&mut reader, &mut writer, "not-found-domain.com".to_string());

        assert!(result.is_ok());

        writer.set_position(0);

        let mut data = Vec::new();
        let result = writer.read_to_end(&mut data);

        assert!(result.is_ok());
        assert!(data.is_empty());
    }

    #[test]
    fn test_remove_command_multiple_domains_on_the_same_line() {
        let mut reader = Cursor::new(Vec::from(
            "# here is my comment\n127.0.0.1 localhost\n127.0.0.1 one-more-domain.com other-domain.com\n".to_string(),
        ));
        let mut writer = Cursor::new(Vec::new());

        let result = execute(&mut reader, &mut writer, "one-more-domain.com".to_string());

        assert!(result.is_ok());

        writer.set_position(0);

        let mut data = Vec::new();
        let result = writer.read_to_end(&mut data);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(data).unwrap(),
            "# here is my comment\n127.0.0.1 localhost\n127.0.0.1  other-domain.com\n".to_string()
        );
    }
}
