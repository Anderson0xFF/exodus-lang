#![allow(non_camel_case_types)]

use super::types::Type;
use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    Let,
    If,
    Else,
    Func,
}

#[derive(Logos, PartialEq, Clone)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    Identifier(String),
    #[regex("-?[0-9]+", |lexer| lexer.slice().parse())]
    Integer(i64),
    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+", |lexer| lexer.slice().parse())]
    Floating(f64),
    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    String(String),
    #[regex("(true|false)", |lex| {
        match lex.slice(){
            "true" => true,
            "false" => false,
            _=> todo!()
        }
    })]
    Boolean(bool),
    #[regex("let|if|else|func", |lex|{
        match lex.slice() {
            "let" => Keywords::Let,
            "if" => Keywords::If,
            "else" => Keywords::Else,
            "func" => Keywords::Func,
            _=> todo!()
        }
    })]
    Keyword(Keywords),
    #[regex("(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|string|char|bool)", |lex|{
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
    })]
    Type(Type),
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("@")]
    Atsign,
    #[token("(")]
    LP,
    #[token(")")]
    RP,
    #[token("{")]
    LB,
    #[token("}")]
    RB,
    #[regex("\\^|\\+|\\-|/|%|\\&|<|>|\\&&|==|!=|\\*|\\|\\||!||->|=|::", |lex|{
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
            "&" => Operator::ADDRESSING,
            "!" => Operator::NOT,
            "->" => Operator::ACCESS_OBJECT,
            "::" => Operator::FIND,
            "=" => Operator::ASSIGNMENT,
            _ => todo!(),
        }
    })]
    Operator(Operator),
    #[token("\n")]
    Line,
    #[regex(r"[ \t\f]")]
    Space,
    #[error]
    Error,
    #[token(r"[\3]")]
    EOF,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(identifier) => write!(f, "{identifier}"),
            Token::String(value) => write!(f, "\"{value}\""),
            Token::Type(arg0) => f.debug_tuple("Type").field(arg0).finish(),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Atsign => write!(f, "@"),
            Token::LP => write!(f, "("),
            Token::RP => write!(f, ")"),
            Token::LB => write!(f, "{{"),
            Token::RB => write!(f, "}}"),
            Token::Integer(value) => write!(f, "{value}"),
            Token::Operator(value) => write!(f, "{:?}", value),
            Token::Error => write!(f, "Error"),
            Token::EOF => write!(f, "eof"),
            Token::Boolean(value) => write!(f, "{value}"),
            Token::Line => write!(f, "\n"),
            Token::Space => write!(f, " "),
            Token::Keyword(kwd) => write!(f, "{:?}", kwd),
            Token::Floating(value) => write!(f, "{value}"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EQUAL,
    LT,
    GT,
    AND,
    NOTEQ,
    OR,
    ADDRESSING,
    NOT,
    ACCESS_OBJECT,
    FIND,
    ASSIGNMENT,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MUL => write!(f, "*"),
            Self::DIV => write!(f, "/"),
            Self::MOD => write!(f, "&"),
            Self::EQUAL => write!(f, "=="),
            Self::LT => write!(f, "<"),
            Self::GT => write!(f, ">"),
            Self::AND => write!(f, "&&"),
            Self::NOTEQ => write!(f, "!="),
            Self::OR => write!(f, "||"),
            Self::ADDRESSING => write!(f, "&"),
            Self::NOT => write!(f, "!"),
            Self::ACCESS_OBJECT => write!(f, "->"),
            Self::ASSIGNMENT => write!(f, "="),
            Self::FIND => write!(f, "::"),
        }
    }
}
