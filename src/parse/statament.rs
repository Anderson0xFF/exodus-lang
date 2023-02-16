use super::{errors::ErrorCode, Parse};
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

    FUNC {
        name: String,
        parms: Vec<(String, Type)>,
        body: Vec<Stantament>,
        return_type: Option<Type>,
    },
}

impl std::fmt::Debug for Stantament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stantament::LET { name, typed, expr } => {
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
            } => write!(f, "func {name}({:?}) -> {:?} {{\n    {:?}\n}}\n", parms, return_type, body),
        }
    }
}

impl Stantament {
    pub fn new_let(parse: &mut Parse) -> Stantament {
        let name = match parse.next() {
            Token::Identifier(name) => name,
            _ => parse.fatal(ErrorCode::STATUS_VARIABLE_ERROR, format!("expected identifier after 'let'")),
        };

        let typed = Stantament::get_type(parse);

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

    pub fn new_func(parse: &mut Parse) -> Stantament {
        let name = match parse.next() {
            Token::Identifier(name) => name,
            _ => parse.fatal(ErrorCode::STATUS_FUNCTION_PROTO_ERROR, format!("expected identifier after 'func'")),
        };

        parse.check(Token::LP);

        let mut parms = Vec::new();

        loop {

            match parse.next() {
                Token::Identifier(parm_name) => match Self::get_type(parse) {
                    Some(typed) => parms.push((parm_name, typed)),
                    _ => parse.fatal(ErrorCode::STATUS_PARAMETRE_TYPE, format!("expected parametre type after 'func {name}({parm_name}: ..?)' ")),
                },
                Token::Comma => (),
                Token::RP => break,
                _=> ()
            }

            match parse.next() {
                Token::Comma => (),
                Token::RP => break,
                token => parse.unexpected(token)
            }
        }

        parse.check(Token::LB);

        let mut body = Vec::new();
        loop {
            match parse.next() {
                Token::Let => body.push(Stantament::new_let(parse)),
                Token::If => body.push(Stantament::new_if(parse)),
                Token::RB => break,
                tk => parse.unexpected(tk),
            }
        }

        Stantament::FUNC { name, parms, body, return_type: None }
    }

    fn get_type(parse: &mut Parse) -> Option<Type> {
        match parse.next() {
            Token::Colon => (),
            _ => {
                parse.back();
                return None;
            }
        }

        match parse.next() {
            Token::Type(typed) => Some(typed),
            Token::Identifier(id) => Some(Type::Object(id)),
            _ => parse.fatal(
                ErrorCode::STATUS_FAILED_TYPING,
                format!("expected type after ':'"),
            ),
        }
    }
}
