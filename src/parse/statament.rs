use super::Parse;
use crate::lexer::tokens::{Expression, Operator, Token, Type};

pub enum Stantament {
    LET {
        name: String,
        typed: Option<Type>,
        expr: Vec<Expression>,
    },

    IF {
        condition: Vec<Expression>,
        then: Vec<Stantament>,
        or: Option<Vec<Stantament>>,
    },
}

impl std::fmt::Debug for Stantament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LET { name, typed, expr } => {
                write!(f, "let {} <{:?}> = {:?};", name, typed, expr)
            }
            Self::IF {
                condition,
                then,
                or,
            } => write!(
                f,
                "if({:?}) {{\n    {:#?}\n}} else {{\n    {:?}\n}}",
                condition, then, or
            ),
        }
    }
}

impl Stantament {
    pub fn new_let(parse: &mut Parse) -> Stantament {
        let name = match parse.next() {
            Token::Identifier(name) => name,
            t => parse.syntax_error(t, Token::Identifier(String::from("Name"))),
        };

        let typed = match parse.next() {
            Token::Type(typed) => Some(typed),
            _ => {
                parse.back();
                None
            }
        };

        parse.check(Token::Operator(Operator::ASSIGNMENT));

        let mut expr = Vec::new();
        loop {
            let value = parse.check_value();
            match parse.next() {
                Token::Operator(op) => {
                    expr.push(Expression::Unary {
                        operator: Some(op),
                        value,
                    });
                }
                Token::Semicolon => {
                    expr.push(Expression::Unary {
                        operator: None,
                        value,
                    });
                    break;
                }
                tk => parse.syntax_error(tk, Token::Semicolon),
            };
        }

        Stantament::LET { name, typed, expr }
    }

    pub fn new_if(parse: &mut Parse) -> Stantament {
        parse.check(Token::LP);

        let mut condition = Vec::new();
        loop {
            let value = parse.check_value();
            match parse.next() {
                Token::Operator(op) => {
                    condition.push(Expression::Unary {
                        operator: Some(op),
                        value,
                    });
                }
                Token::RP => {
                    condition.push(Expression::Unary {
                        operator: None,
                        value,
                    });
                    break;
                }
                tk => parse.syntax_error(tk, Token::RP),
            };
        }

        parse.check(Token::LB);

        let mut then = Vec::new();
        loop {
            match parse.next() {
                Token::Let => then.push(Stantament::new_let(parse)),
                Token::If => then.push(Stantament::new_if(parse)),
                Token::RB => break,
                tk => parse.unexpected(tk),
            }
        }

        let or = match parse.next() {
            Token::Else => {
                parse.check(Token::LB);
                let mut _else = Vec::new();

                loop {
                    match parse.next() {
                        Token::Let => _else.push(Stantament::new_let(parse)),
                        Token::If => _else.push(Stantament::new_if(parse)),
                        Token::RB => break,
                        tk => parse.unexpected(tk),
                    }
                }
                Some(_else)
            }
            _ => {
                parse.back();
                None
            }
        };

        Stantament::IF {
            condition,
            then,
            or,
        }
    }
}
