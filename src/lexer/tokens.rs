#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::parse::values::Value;
use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Boolean,
    Char,
    String,
    Object(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Unary {
        value: Value,
        operator: Option<Operator>,
    },
    Binary {
        x: Box<Expression>,
        operator: Operator,
        y: Box<Expression>,
    },
}

#[derive(Logos, PartialEq, Clone)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    Identifier(String),

    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    String(String),

    #[regex("(true|false)", |lex| {
        match lex.slice(){
            "true" => true,
            "false" => false,
            _=> todo!()
        }
    })]
    Bool(bool),
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("func")]
    Func,
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
    #[regex("-?[0-9]+", |lexer| lexer.slice().parse())]
    IntValue(i64),

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
    LINE,

    #[regex(r"[ \t\f]")]
    SPACE,

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
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
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
            Token::IntValue(value) => write!(f, "{value}"),
            Token::Operator(value) => write!(f, "{:?}", value),
            Token::Error => write!(f, "Error"),
            Token::EOF => write!(f, "eof"),
            Token::Bool(value) => write!(f, "{value}"),
            Token::LINE => write!(f, "\n"),
            Token::SPACE => write!(f, " "),
            Token::Func => write!(f, "func"),
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
