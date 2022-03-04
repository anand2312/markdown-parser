use std::fs;
mod lexer;

fn main() {
    let content = fs::read_to_string("README.md").expect("Couldn't read file.");
    println!("{}", &content);
    let lexer = lexer::Lexer::new(content.as_str());

    for token in lexer.into_iter() {
        println!("{:?}", token);
    }
}
