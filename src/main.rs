use std::fs;
mod lexer;
mod parser;

fn main() {
    let _content = fs::read_to_string("test.md").expect("Couldn't read file.");
    let mut parser = parser::Parser::new("# Hello world\n");

    println!("{:?}", &parser.lexer_tokens);

    let hashtag = parser.parse_hashtag();

    match hashtag {
        Some(b) => {
            println!("{:?}", b.children)
        }
        None => {
            println!("Not a header")
        }
    }
}
