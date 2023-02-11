#![allow(dead_code)]

use super::{Token, Operator};
use logos::Lexer;

impl Token {
    pub fn variable(lex: &mut Lexer<Token>) -> Vec<Token> {
        let mut body = Vec::new();
        loop {
            match lex.next() {
                Some(v) => match v {
                    Token::Space => (),
                    Token::Assing => (),
                    Token::IntValue(_) => body.push(v),
                    Token::Semicolon => break,
                    Token::Operator(op) => body.push(Token::Operator(op)),
                    Token::Identifier(_) => body.push(v),
                    _ => break,
                },
                _ => break,
            }
        }
        return body;
    }

    pub fn operator(lex: &mut Lexer<Token>) -> Operator {
        match lex.slice() {
            "+" => Operator::ADD,
            "-" => Operator::SUB,
            "*" => Operator::MUL,
            "/" => Operator::DIV,
            "%" => Operator::MOD,
            _=> todo!()
        }
    }

    pub fn get_operator(&self) -> Option<Operator>{
        match self {
            Token::Operator(token) =>  Some(token.clone()),
            _ => None,
        }
    }

}
