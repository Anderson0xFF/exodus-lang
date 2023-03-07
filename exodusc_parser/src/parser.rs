use crate::{
    errors::SyntaxErrors::{self, *},
    expressions::Expression,
    stantaments::Stantament,
};
use colored::Colorize;
use exodusc_lexer::{scanner, types::Type, Lexer};
use std::collections::HashMap;

pub type AST = Vec<Stantament>;

pub struct Parser {
    lexer: Lexer,
    ast: AST,
}

impl Parser {
    pub fn default(lexer: Lexer) -> Self {
        let ast = AST::new();
        Self { lexer, ast }
    }

    pub fn analyse(&mut self) -> AST {
        loop {
            match self.lexer.next() {
                scanner::Token::Keyword(scanner::Keywords::Func) => self.analyse_func(),
                scanner::Token::EOF => break,
                token => self.unexpected(token),
            };
        }
        self.ast.clone()
    }

    fn analyse_func(&mut self) {
        let line = self.lexer.line();

        let name = match self.lexer.next() {
            scanner::Token::Identifier(name) => name,
            _ => self.report(
                SYNTAX_EXPECTED_NAME,
                format!("expected `name` after `func`"),
            ),
        };

        self.check_token(scanner::Token::LParen);

        let mut parms = HashMap::new();
        loop {
            let parm = match self.lexer.next() {
                scanner::Token::Identifier(parm) => parm,
                scanner::Token::Comma => continue,
                scanner::Token::RParen => break,
                token => self.unexpected(token),
            };

            match self.get_type() {
                Type::Void => {
                    let description = format!("missing `type`");
                    self.report(SYNTAX_MISSING_TYPE, description);
                }
                typedef => parms.insert(parm, typedef)
            };
            

            match self.lexer.next() {
                scanner::Token::Comma => (),
                scanner::Token::RParen => break,
                token => self.unexpected(token),
            }
        }

        let return_type = self.get_type();
        let body = self.analyse_scope();

        self.ast.push(Stantament::Func { name, parms, body, return_type, line })
    }

    fn analyse_scope(&mut self) -> Vec<Stantament> {
        self.check_token(scanner::Token::LBrace);
        let mut body = Vec::new();
        loop {
            match self.lexer.next() {
                scanner::Token::Keyword(scanner::Keywords::Let) => body.push(self.analyse_let()),
                scanner::Token::Keyword(scanner::Keywords::If) => body.push(self.analyse_if_else()),
                scanner::Token::Keyword(scanner::Keywords::While) => body.push(self.analyse_while()),
                scanner::Token::Keyword(scanner::Keywords::Return) => body.push(self.analyse_return()),
                scanner::Token::RBrace => break body,
                token => self.unexpected(token),
            };
        }
    }
    
    fn analyse_let(&mut self) -> Stantament {
        let name = match self.lexer.next() {
            scanner::Token::Identifier(name) => name,
            _ => {
                let error_msg = format!("expected a name after `let` keyword on line {}", self.lexer.line());
                self.report(SYNTAX_EXPECTED_NAME, error_msg)
            }
        };
    
        let typedef = self.get_type();
    
        self.check_token(scanner::Token::Operator(scanner::Operator::ASSIGNMENT));
    
        let expr = self.parse_exprs(scanner::Token::Semicolon);
        let line = self.lexer.line();
    
        Stantament::Let {
            name,
            typedef,
            expr,
            line,
        }
    }

    fn analyse_if_else(&mut self) -> Stantament{
        let line = self.lexer.line();
        self.check_token(scanner::Token::LParen);

        let condition = self.parse_exprs(scanner::Token::RParen);

        let then = self.analyse_scope();

        let or = match self.lexer.peek() {
            scanner::Token::Keyword(scanner::Keywords::Else) => {
                self.lexer.skip();
                self.analyse_scope()
            },
            _=> Vec::new()
        };

        Stantament::If { condition, then, or, line }
    }
    
    fn analyse_while(&mut self) -> Stantament {
        let line = self.lexer.line();
        self.check_token(scanner::Token::LParen);

        let condition = self.parse_exprs(scanner::Token::RParen);

        let body = self.analyse_scope();

        Stantament::While { condition, body, line }
    }

    fn analyse_return(&mut self) -> Stantament {
        let expr = self.parse_exprs(scanner::Token::Semicolon);

        Stantament::Return { expr }
    }

    fn parse_exprs(&mut self, end: scanner::Token) -> Vec<Expression>{
        let mut expr = Vec::new();
        let x = self.parse_value();
    
        let mut operator = None;
        match self.lexer.next() {
            scanner::Token::Operator(op) => {
                operator = Some(op);
            },
            scanner::Token::Semicolon => {
                expr.push(Expression::Unary { operator, value: x.unwrap() });
                return expr;
            }
            _ => ()
        };
    
        let y = self.parse_value();
    
        if x.is_some() && y.is_some() {
            expr.push(Expression::Binary { x: x.unwrap(), operator: operator.unwrap(), y: y.unwrap() });
        } else if x.is_none() && y.is_some() {
            expr.push(Expression::Unary { operator, value: y.unwrap() });
        }
    
        if self.lexer.peek() != end {
            expr.extend(self.parse_exprs(end));
        }
        self.lexer.skip();

        return expr;
    }
    
    fn parse_value(&mut self) -> Option<scanner::Literal> {
        match self.lexer.next() {
            scanner::Token::Literal(literal) => Some(literal),
            scanner::Token::Identifier(var) => Some(scanner::Literal::Var(var)),
            scanner::Token::Operator(_) => {
                self.lexer.back();
                None
            },
            token => {
                let expected_value_msg = format!("expected a value before `{}`", token);
                self.report(SYNTAX_EXPECTED_VALUE, expected_value_msg)
            }
        }
    }
    
    fn check_token(&mut self, token: scanner::Token) {
        if self.lexer.peek() == token{
            self.lexer.skip();
            return;
        }

        let back = self.lexer.next();
        self.report(SYNTAX_EXPECTED_TOKEN,format!("expected `{}` after `{}`. ", token, back));
    }

    fn get_type(&mut self) -> Type {
        match self.lexer.peek() {
            scanner::Token::Colon => self.lexer.skip(),
            _=> return Type::Void
        }

        match self.lexer.next() {
            scanner::Token::Type(typed) => typed,
            scanner::Token::Identifier(id) => Type::Object(id),
            token => self.report(
                SYNTAX_EXPECTED_TYPE,
                format!("expected `type` after `:`, found `{}`", token),
            ),
        }
    }

    fn unexpected(&self, token: scanner::Token) -> ! {
        self.report(
            SYNTAX_UNEXPECTED_TOKEN, format!("unexpected token `{}`", token),
        )
    }

    fn report(&self, status: SyntaxErrors, description: String) -> ! {
        let filename = self.lexer.filename();
        let lines = self.lexer.line();
        let column_number = self.lexer.column();
        let max_digits = (lines as f64).log10().floor() as usize + 1;
        let padded_number = format!("{:>0width$}", lines, width=max_digits);

        let source = self.lexer.get_source_line(lines).unwrap();
        let space = " ".repeat(lines.to_string().len());

        println!("{space}--> {}:{}:{}",
                 filename.bold().blue(),
                 padded_number.bright_white(),
                 column_number.to_string().bright_white()
        );


        println!("{space}|");
        println!("{number}| {source}", number = padded_number.bright_cyan(), source = source.trim_end().bright_red());
        println!("{space}|{arrow}\x1b[91m^\x1b[0m", arrow = " ".repeat(column_number));
        println!("{space}|{arrow}\x1b[91m|\x1b[0m", arrow = " ".repeat(column_number));
        println!("{space}| [{}]\x1b[91m error: {}\x1b[0m",  status.code().bright_red().bold(), description.bright_red());
        std::process::exit(1);
    }
    
    pub fn reset(&mut self) {
        self.lexer.reset();
        self.ast.clear();
    }
}
