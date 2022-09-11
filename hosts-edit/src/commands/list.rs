use std::error::Error;
use std::io::Read;

use hoster::cst::CstNode;
use hoster::parser::Parser;
use hoster::tokenizer::Tokenizer;
use hoster::visitor::CstVisitor;

pub(crate) fn execute<R>(reader: &mut R, _with_comments: bool) -> Result<(), Box<dyn Error>>
where
    R: Read,
{
    let tokens = Tokenizer::new_with_reader(reader).parse()?.get_tokens();

    println!("{:#?}", tokens);

    let mut parser = Parser::builder()
        .visitor(Visitor {
            print_new_line: false,
        })
        .build();

    let cst = parser.parse::<1>(tokens)?;

    println!("{:#?}", cst);

    parser.visit(&cst);
    Ok(())
}

pub(crate) struct Visitor {
    print_new_line: bool,
}

impl CstVisitor for Visitor {
    fn visit(&mut self, _idx: usize, node: &CstNode) -> Option<()> {
        match node {
            CstNode::Host(host) => {
                self.print_new_line = true;
                print!("{}  ", host);
            }
            CstNode::IP(ip) => {
                print!("{}  ", ip);
                self.print_new_line = false;
            }
            CstNode::NewLine => {
                if self.print_new_line {
                    self.print_new_line = false;
                    println!();
                }
            }
            _ => {}
        }

        Some(())
    }
}
