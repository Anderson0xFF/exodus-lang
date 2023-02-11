use logos::Logos;
pub mod implementation;


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    Identifier(String),

    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    String(String),

    #[token("let" , Token::variable)]
    Let(Vec<Token>),

    #[token("if" , Token::variable)]
    If(Vec<Token>),

    #[token(" ")]
    Space,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Assing,
    #[token("(")]
    LP,
    #[token(")")]
    RP,
    #[token("{")]
    LB,
    #[token("}")]
    RB,

    #[regex("-?[0-9]+", |lexer| lexer.slice().parse())]
    IntValue(i64),

    #[regex("\\^|\\+|\\-|/|%|\\&|<|>|\\&&|==|!=|\\*|\\|\\||!", Token::operator)]
    Operator(Operator),

    #[error]
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    ADD, 
    SUB,
    MUL,
    DIV,
    MOD,
}