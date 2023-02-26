use std::collections::HashMap;

use super::{errors::ErrorCode, Parse};
use crate::lexer::{tokens::{Expression, Operator, Keywords}, types::Type};


#[derive(Clone)]
pub enum Stantament {
    LET {
        name: String,
        typed: Type,
        expr: Vec<Expression>,
        line: usize
    },

    IF {
        condition: Vec<Expression>,
        then: Vec<Stantament>,
        or: Vec<Stantament>,
    },

    FUNC {
        name: String,
        parms: HashMap<String, Type>,
        body: Vec<Stantament>,
        return_type: Type,
        line: usize
    },
}

impl std::fmt::Debug for Stantament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stantament::LET { name, typed, expr, line } => {
                write!(f, "let {} <{:?}> = {:?};\n", name, typed, expr)
            }
            Stantament::IF {
                condition,
                then,
                or,
            } => write!(
                f,
                "if({:?}) {{\n    {:#?}\n}} else {{\n    {:?}\n}}",
                condition, then, or
            ),
            Stantament::FUNC {
                name,
                parms,
                body,
                return_type,
                line,
            } => write!(f, "func {name}({:?}) -> {:?} {{\n    {:?}\n}}\n", parms, return_type, body),
        }
    }
}

impl Stantament {
    pub fn create_let(parse: &mut Parse) -> Stantament {
        let name = match parse.next() {
            Keywords::Identifier(name) => name,
            _ => parse.report(ErrorCode::STATUS_VARIABLE_ERROR, format!("expected identifier after 'let'")),
        };

        let typed = Stantament::get_type(parse);

        parse.check(Keywords::Operator(Operator::ASSIGNMENT));

        let mut expr = Vec::new();
        loop {
            let value = parse.check_value();
            match parse.next() {
                Keywords::Operator(op) => expr.push(Expression::Unary {
                    operator: Some(op),
                    value,
                }),
                Keywords::Semicolon => {
                    expr.push(Expression::Unary {
                        operator: None,
                        value,
                    });
                    break;
                }
                tk => parse.syntax_error(tk, Keywords::Semicolon),
            };
        }

        Stantament::LET { name, typed, expr, line: parse.line }
    }

    pub fn create_if(parse: &mut Parse) -> Stantament {
        parse.check(Keywords::LP);

        let mut condition = Vec::new();
        loop {
            let value = parse.check_value();
            match parse.next() {
                Keywords::Operator(op) => condition.push(Expression::Unary {
                    operator: Some(op),
                    value,
                }),
                Keywords::RP => {
                    condition.push(Expression::Unary {
                        operator: None,
                        value,
                    });
                    break;
                }
                tk => parse.syntax_error(tk, Keywords::RP),
            };
        }

        parse.check(Keywords::LB);

        let mut then = Vec::new();
        loop {
            match parse.next() {
                Keywords::Let => then.push(Stantament::create_let(parse)),
                Keywords::If => then.push(Stantament::create_if(parse)),
                Keywords::RB => break,
                tk => parse.unexpected(tk),
            }
        }
        
        let mut or = Vec::new();
        match parse.next() {
            Keywords::Else => {
                parse.check(Keywords::LB);

                loop {
                    match parse.next() {
                        Keywords::Let => or.push(Stantament::create_let(parse)),
                        Keywords::If => or.push(Stantament::create_if(parse)),
                        Keywords::RB => break,
                        tk => parse.unexpected(tk),
                    }
                };
            }
            _ => {parse.back();},
        };

        Stantament::IF {
            condition,
            then,
            or,
        }
    }

    pub fn create_function(parse: &mut Parse) -> Stantament {
        let name = match parse.next() {
            Keywords::Identifier(name) => name,
            _ => parse.report(ErrorCode::STATUS_FUNCTION_PROTO_ERROR, format!("expected identifier after 'func'")),
        };

        parse.check(Keywords::LP);

        let mut parms = HashMap::new();
        loop {

            match parse.next() {
                Keywords::Identifier(parm_name) => match Self::get_type(parse) {
                    Type::Any => parse.report(ErrorCode::STATUS_PARAMETRE_TYPE, format!("expected parametre type after 'func {name}({parm_name}: ..?)' ")),
                    typed => {parms.insert(parm_name, typed);},
                },
                Keywords::Comma => (),
                Keywords::RP => break,
                _=> ()
            }

            match parse.next() {
                Keywords::Comma => (),
                Keywords::RP => break,
                token => parse.unexpected(token)
            }
        };

        parse.check(Keywords::LB);

        let mut body = Vec::new();
        loop {
            match parse.next() {
                Keywords::Let => body.push(Stantament::create_let(parse)),
                Keywords::If => body.push(Stantament::create_if(parse)),
                Keywords::RB => break,
                tk => parse.unexpected(tk),
            }
        }
        Stantament::FUNC { name, parms, body, return_type: Type::Any, line: parse.line }
    }


    fn get_type(parse: &mut Parse) -> Type {
        match parse.next() {
            Keywords::Colon => (),
            _ => {
                parse.back();
                return Type::Any;
            }
        }

        match parse.next() {
            Keywords::Type(typed) => typed,
            Keywords::Identifier(id) => Type::Object(id),
            _ => parse.report(
                ErrorCode::STATUS_FAILED_TYPING,
                format!("expected type after ':'"),
            ),
        }
    }
}
