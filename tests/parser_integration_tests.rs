use markdown_parser::{Block, ParsedToken, Parser};

#[test]
fn test_header_endswithnewline() {
    let mut parser = Parser::new("# header\n");
    let expected = Block {
        children: vec![ParsedToken::Header {
            level: 1,
            content: " header".to_owned(),
        }],
    };
    assert_eq!(parser.parse_hashtag().unwrap(), expected);
}

#[test]
fn test_header_endswith_eof() {
    let mut parser = Parser::new("## ends without newline");
    let expected = Block {
        children: vec![ParsedToken::Header {
            level: 2,
            content: " ends without newline".to_owned(),
        }],
    };
    assert_eq!(parser.parse_hashtag().unwrap(), expected);
}

#[test]
fn test_header_inbetweencontent() {
    let mut parser = Parser::with_current_at("hi # test fails", 1);
    assert!(matches!(parser.parse_hashtag(), None));
}
