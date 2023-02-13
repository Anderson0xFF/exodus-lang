use logos::Logos;
pub mod implementation;

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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Let {
    name: String,
    typed: Option<Type>,
    expr: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    operator: Option<Operator>,
    value: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    x: Unary,
    operator: Operator,
    y: Unary,
}

impl Binary {
    pub fn new(x: Unary, operator: Operator, y: Unary) -> Self {
        Self { x, operator, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Unary(Unary),
    Binary(Binary),
}

impl Expression {
    pub fn bind_expression(exprs: Vec<Expression>) -> Vec<Expression> {
        if exprs.len() == 1 {
            return exprs;
        }

        let mut expr = Vec::new();
        let mut intter = exprs.iter();

        loop {
            let x = match intter.next() {
                Some(exp) => match exp {
                    Expression::Unary(u) => u.clone(),
                    _=> panic!()
                },
                None => break,
            };
            let y = match intter.next() {
                Some(exp) => match exp {
                    Expression::Unary(u) => u.clone(),
                    _=> panic!()
                },
                None => {
                    expr.push(Expression::Unary(x));
                    break;
                }
            };

            let operator = y.clone().operator.unwrap();

            let binary = Binary::new(x, operator, y);
            expr.push(Expression::Binary(binary));
        }
        return expr;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    CHAR(char),
    STRING(String),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    REF(String),
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    Identifier(String),

    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    String(String),

    #[token("let", Let::default)]
    Let(Let),

    #[regex(
        "(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|string|char|bool)",
        Type::default
    )]
    Type(Type),

    #[token("if", Token::if_conditional)]
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
    #[token("\n")]
    LINEBREAK,
}

#[derive(Debug, PartialEq, Clone)]
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
    REF,
    NOT,
}
