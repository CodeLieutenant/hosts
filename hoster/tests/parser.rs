#[cfg(test)]
use std::fs::File;

use hoster::cst::CstNode;
use hoster::parser::Parser;
use hoster::tokenizer::Tokenizer;
use hoster::tokens::Tokens;

#[test]
fn test_tokenizer_and_parser_utf16_with_bom_bytes() {
    use smallvec::smallvec_inline;

    let mut file = File::open("tests/data/utf16-hosts-with-bom-bytes").unwrap();

    let tokenizer = Tokenizer::new_with_reader(&mut file);

    let tokens = tokenizer.parse();

    assert!(tokens.is_ok());
    let tokens = tokens.unwrap().get_tokens();

    assert_eq!(
        vec![
            Tokens::Comment(" Copyright (c) 1993-2009 Microsoft Corp.".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " This is a sample HOSTS file used by Microsoft TCP/IP for Windows.".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " This file contains the mappings of IP addresses to host names. Each".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " entry should be kept on an individual line. The IP address should".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " be placed in the first column followed by the corresponding host name."
                    .to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " The IP address and the host name should be separated by at least one".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" space.".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " Additionally, comments (such as these) may be inserted on individual".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " lines or following the machine name denoted by a '#' symbol.".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" For example:".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                "      102.54.94.97     rhino.acme.com          # source server".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                "       38.25.63.10     x.acme.com              # x client host".to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" localhost name resolution is handled within DNS itself.".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("\t127.0.0.1       localhost".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment("\t::1             localhost".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" Added by Docker Desktop".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("192.168.0.17".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("host.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("192.168.0.17".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("gateway.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(
                " To allow the same kube context to work on the host and the container:"
                    .to_string()
            ),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::HostOrIp("127.0.0.1".to_string()),
            Tokens::Space,
            Tokens::HostOrIp("kubernetes.docker.internal".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
            Tokens::Comment(" End of section".to_string()),
            Tokens::CarriageReturn,
            Tokens::NewLine,
        ],
        tokens
    );

    let parser: Parser<()> = Parser::builder().build();

    let cst = parser.parse::<100>(tokens);

    assert!(cst.is_ok());
    let cst = cst.unwrap();

    assert_eq!(
        smallvec_inline![
            CstNode::Comment(" Copyright (c) 1993-2009 Microsoft Corp.".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " This is a sample HOSTS file used by Microsoft TCP/IP for Windows.".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " This file contains the mappings of IP addresses to host names. Each".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " entry should be kept on an individual line. The IP address should".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " be placed in the first column followed by the corresponding host name."
                    .to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " The IP address and the host name should be separated by at least one".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(" space.".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " Additionally, comments (such as these) may be inserted on individual".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " lines or following the machine name denoted by a '#' symbol.".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(" For example:".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                "      102.54.94.97     rhino.acme.com          # source server".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                "       38.25.63.10     x.acme.com              # x client host".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " localhost name resolution is handled within DNS itself.".to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("\t127.0.0.1       localhost".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment("\t::1             localhost".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(" Added by Docker Desktop".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::IP("192.168.0.17".parse().unwrap()),
            CstNode::Space,
            CstNode::Host("host.docker.internal".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::IP("192.168.0.17".parse().unwrap()),
            CstNode::Space,
            CstNode::Host("gateway.docker.internal".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(
                " To allow the same kube context to work on the host and the container:"
                    .to_string()
            ),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::IP("127.0.0.1".parse().unwrap()),
            CstNode::Space,
            CstNode::Host("kubernetes.docker.internal".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
            CstNode::Comment(" End of section".to_string()),
            CstNode::CarriageReturn,
            CstNode::NewLine,
        ],
        cst.nodes
    );
}
