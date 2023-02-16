use logos::Logos;
use parse::Parse;
use std::fs;

mod lexer;
mod parse;

fn main() {
    let source = fs::read_to_string("./main.exo").unwrap();
    let lexer = lexer::tokens::Token::lexer(&source);
    let mut parse = Parse::new(String::from("main.exo"), lexer);
    let ast = parse.analyse();
    println!("{:#?}", ast);
}
