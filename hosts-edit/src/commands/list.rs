use std::error::Error;
use std::io::{Read, Write};

use hoster::cst::CstNode;
use hoster::parser::Parser;
use hoster::tokenizer::Tokenizer;
use hoster::visitor::CstVisitor;

pub(crate) fn execute<R, W>(
    reader: &mut R,
    output: &mut W,
    _with_comments: bool,
) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let tokens = Tokenizer::new_with_reader(reader).parse()?.get_tokens();

    let mut parser = Parser::builder()
        .visitor(Visitor {
            print_new_line: false,
            output,
        })
        .build();

    let cst = parser.parse::<1>(tokens)?;

    parser.visit(&cst);
    Ok(())
}

pub(crate) struct Visitor<W> {
    print_new_line: bool,
    output: W,
}

impl<W> CstVisitor for Visitor<W>
where
    W: Write,
{
    fn visit(&mut self, _idx: usize, node: &CstNode) -> Option<()> {
        match node {
            CstNode::Host(host) => {
                self.print_new_line = true;
                write!(self.output, "\t{}", host).unwrap();
            }
            CstNode::IP(ip) => {
                write!(self.output, "{}", ip).unwrap();

                self.print_new_line = false;
            }
            CstNode::NewLine => {
                if self.print_new_line {
                    self.print_new_line = false;
                    writeln!(self.output).unwrap();
                }
            }
            _ => {}
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_list_without_comments() {
        let mut output = Cursor::new(Vec::new());

        let mut reader = Cursor::new(
            "\
# localhost name resolution is handled within DNS itself.
# Added by Docker Desktop
192.168.0.17\thost.docker.internal
192.168.0.17 gateway.docker.internal
# To allow the same kube context to work on the host and the container:
\t127.0.0.1\tkubernetes.docker.internal


# End of section
"
            .to_string(),
        );

        let result = execute(&mut reader, &mut output, false);

        assert!(result.is_ok());

        let output = String::from_utf8(output.into_inner()).unwrap();

        assert_eq!(
            "\
192.168.0.17\thost.docker.internal
192.168.0.17\tgateway.docker.internal
127.0.0.1\tkubernetes.docker.internal
"
            .to_string(),
            output
        )
    }
}
