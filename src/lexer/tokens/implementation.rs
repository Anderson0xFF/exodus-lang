#![allow(dead_code)]

use crate::lexer::tokens::{Unary, Value};

use super::{Expression, Let, Operator, Token, Type};
use logos::Lexer;

impl Let {
    pub fn default(lex: &mut Lexer<Token>) -> Let {
        let name = match Token::next_valided_token(lex) {
            Token::Identifier(name) => name,
            token => Token::expected_panic(token, Token::Identifier("'varname'".to_owned())),
        };

        Token::expected(lex, Token::Assing);
        let mut expr = Vec::new();
        let mut operator: Option<Operator> = None;

        loop {
            match Token::next_valided_token(lex) {
                Token::String(value) => expr.push(Expression::Unary(Unary {
                    operator: operator.clone(),
                    value: Value::STRING(value),
                })),
                Token::IntValue(value) => expr.push(Expression::Unary(Unary {
                    operator: operator.clone(),
                    value: Value::I32(value as i32),
                })),
                Token::Operator(op) => {
                    operator = Some(op);
                }
                Token::Identifier(value) => expr.push(Expression::Unary(Unary {
                    operator: operator.clone(),
                    value: Value::REF(value),
                })),
                Token::Semicolon => break,
                _ => (),
            }
        }

        let expr = Expression::bind_expression(expr);

        Self {
            name,
            typed: None,
            expr,
        }
    }
}

impl Type {
    pub fn default(lex: &mut Lexer<Token>) -> Type {
        match lex.slice() {
            "i8" => Type::I8,
            "i16" => Type::I16,
            "i32" => Type::I32,
            "i64" => Type::I64,
            "u8" => Type::U8,
            "u16" => Type::U16,
            "u32" => Type::U32,
            "u64" => Type::U64,
            "f32" => Type::F32,
            "f64" => Type::F64,
            "char" => Type::Char,
            "string" => Type::String,
            "bool" => Type::Boolean,
            _ => todo!(),
        }
    }
}

impl Token {
    pub fn expr(lex: &mut Lexer<Token>) -> Expression {
        println!("{:?}", lex);
        todo!()
    }

    pub fn if_conditional(lex: &mut Lexer<Token>) -> Vec<Token> {
        let mut body = Vec::new();
        Self::expected(lex, Token::LP);
        loop {
            let token = Self::next_valided_token(lex);
            match token {
                Token::Identifier(_) => body.push(token),
                //Token::String(_) => body.push(token),
                Token::RP => break,
                //Token::IntValue(_) => body.push(token),
                Token::Operator(_) => body.push(token),
                _ => Self::expected_panic(token, Token::RP),
            }
        }
        return body;
    }

    fn expected(lex: &mut Lexer<Token>, expected: Token) {
        let current = Self::next_valided_token(lex);
        if current != expected {
            Self::expected_panic(current, expected);
        }
    }

    fn expected_panic(current: Token, expected: Token) -> ! {
        panic!("expected '{:?}' got '{:?}'", expected, current);
    }

    fn next_valided_token(lex: &mut Lexer<Token>) -> Token {
        loop {
            if let Some(item) = lex.next() {
                match item {
                    Token::Space => continue,
                    Token::LINEBREAK => continue,
                    Token::Error => panic!("Error token"),
                    _ => break item,
                }
            }
        }
    }

    pub fn operator(lex: &mut Lexer<Token>) -> Operator {
        match lex.slice() {
            "+" => Operator::ADD,
            "-" => Operator::SUB,
            "*" => Operator::MUL,
            "/" => Operator::DIV,
            "%" => Operator::MOD,
            "==" => Operator::EQUAL,
            "<" => Operator::LT,
            ">" => Operator::GT,
            "!=" => Operator::NOTEQ,
            "&&" => Operator::AND,
            "||" => Operator::OR,
            "&" => Operator::REF,
            "!" => Operator::NOT,
            _ => todo!(),
        }
    }

    pub fn get_operator(&self) -> Option<Operator> {
        match self {
            Token::Operator(token) => Some(token.clone()),
            _ => None,
        }
    }
}
