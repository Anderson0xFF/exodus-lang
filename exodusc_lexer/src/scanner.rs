#![allow(non_camel_case_types)]

use super::types::Type;
use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    Let,
    If,
    Else,
    Func,
    Return,
}

impl std::fmt::Display for Keywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keywords::Let => write!(f, "let"),
            Keywords::If => write!(f, "if"),
            Keywords::Else => write!(f, "else"),
            Keywords::Func => write!(f, "func"),
            Keywords::Return => write!(f, "return"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Default,
    Char(char),
    Integer(i32),
    Long(i64),
    Float(f64),
    Double(f64),
    String(String),
    Boolean(bool),
    Var(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Char(c) => write!(f, "{}", c),
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Long(l) => write!(f, "{}", l),
            Literal::Float(flt) => write!(f, "{}", flt),
            Literal::Double(d) => write!(f, "{}", d),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Var(v) => write!(f, "{}", v),
            Literal::Default => write!(f, "??"),
        }
    }
}

#[derive(Logos, PartialEq, Clone, Debug)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    Identifier(String),

    #[regex("-?[0-9]+", |lexer| {
        let number : i64 = lexer.slice().parse().expect("Can't parse number!");
        if number > i32::MAX as i64 {
            return Literal::Long(number);
        }
        return Literal::Integer(number as i32);
    })]
    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+", |lexer| {
        let number : f64 = lexer.slice().parse().expect("Can't parse number!");
        if number > f32::MAX as f64 {
            return Literal::Double(number);
        }
        Literal::Float(number)
    })]
    #[regex(r#""[^"]*""#, |lexer| {
        let string: String = lexer.slice()[1..(lexer.slice().len()-1)].to_owned();
        Literal::String(string)
    })]
    #[regex("(true|false)", |lex| {
        match lex.slice(){
            "true" => Ok(Literal::Boolean(true)),
            "false" => Ok(Literal::Boolean(false)),
            _=> Err(())
        }
    })]
    #[regex(r#"'[^']*'"#, |lexer|{
        let text = &lexer.slice()[1..(lexer.slice().len()-1)];
        Literal::Char(text.chars().next().expect("Invalid character literal"))
    })]
    Literal(Literal),

    #[regex("let|if|else|func|return", |lex|{
        match lex.slice() {
            "let" => Keywords::Let,
            "if" => Keywords::If,
            "else" => Keywords::Else,
            "func" => Keywords::Func,
            "return" => Keywords::Return,
            _ => panic!("Unrecognized Keyword"),
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
            _ => panic!("Unrecognized Type"), // substituindo todo!() por panic!()
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
    AtSign,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
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
            "->" => Operator::ARROW,
            "::" => Operator::NAVIGATION,
            "=" => Operator::ASSIGNMENT,
            _ => panic!("Unrecognized Operator"),
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


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(idenf) => write!(f, "{idenf}"),
            Token::Literal(liteal) => write!(f, "{liteal}"),
            Token::Keyword(keyword) => write!(f, "{keyword}"),
            Token::Type(typedef) => write!(f, "{typedef}"),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::AtSign => write!(f, "@"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Operator(op) => write!(f, "{op}"),
            Token::Line => write!(f, "\\n"),
            Token::Space => write!(f, " "),
            Token::Error => write!(f, ""),
            Token::EOF => write!(f, "end of file"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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
    ARROW,
    NAVIGATION,
    ASSIGNMENT,
}

impl std::fmt::Display for Operator {
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
            Self::ARROW => write!(f, "->"),
            Self::ASSIGNMENT => write!(f, "="),
            Self::NAVIGATION => write!(f, "::"),
        }
    }
}
