use logos::Logos;
use std::fs;

mod lexer;

fn main() {
    let source = fs::read_to_string("./main.exo").unwrap();
    let lexer = lexer::tokens::Token::lexer(&source);
    for value in lexer {
        println!("{:?}", value)
    }
}
