use self::{errors::ErrorCode, statament::Stantament, values::Value};
use crate::lexer::tokens::Token;
use logos::Lexer;
use std::process::exit;

pub mod errors;
pub mod statament;
pub mod values;

pub struct Parse {
    filename: String,
    tokens: Vec<Token>,
    idx: i32,
    space: usize,
    line: usize,
}

impl Parse {
    pub fn new(filename: String, lexer: Lexer<Token>) -> Self {
        let mut tokens = Vec::new();
        for token in lexer {
            tokens.push(token);
        }

        Self {
            filename,
            tokens,
            idx: 0,
            space: 1,
            line: 1,
        }
    }

    pub fn analyse(&mut self) -> Vec<Stantament> {
        let mut stantament = Vec::new();
        loop {
            match self.next() {
                Token::Let => stantament.push(Stantament::new_let(self)),
                Token::If => stantament.push(Stantament::new_if(self)),
                Token::Func => stantament.push(Stantament::new_func(self)),
                Token::EOF => break,
                tk => self.unexpected(tk),
            }
        }
        return stantament;
    }

    fn next(&mut self) -> Token {
        loop {
            if self.idx >= self.tokens.len() as i32 {
                return Token::EOF;
            }

            let current = self.tokens[self.idx as usize].clone();
            if current == Token::LINE {
                self.space = 0;
                self.line += 1;
                self.idx += 1;
            } else if current == Token::SPACE {
                self.space += 1;
                self.idx += 1;
            } else if current == Token::Error {
                self.idx += 1;
                self.space += 1;
            } else {
                self.space += 1;
                self.idx += 1;
                break current;
            }
        }
    }

    fn back(&mut self) -> Token {
        loop {
            if self.idx - 1 > 0 {
                let current = self.tokens[(self.idx - 2) as usize].clone();
                if current == Token::LINE {
                    self.space = 0;
                    self.line -= 1;
                    self.idx -= 1;
                } else if current == Token::SPACE {
                    self.space -= 1;
                    self.idx -= 1;
                } else if current == Token::Error {
                    self.idx -= 1;
                    self.space -= 1;
                } else {
                    self.idx -= 1;
                    self.space -= 1;
                    break current;
                }
            } else {
                break self.tokens[(self.idx - 1) as usize].clone();
            }
        }
    }

    fn check(&mut self, expected: Token) {
        let current = self.next();
        if current != expected {
            self.syntax_error(current, expected);
        }
    }

    pub fn fatal(&self, status: ErrorCode, note: String) -> ! {
        println!(
            "{}[{}:{}] => [ status: {:?}, code: 0x{:X} ]\n    note: {}",
            self.filename,
            self.line,
            self.space,
            status,
            status.code(),
            note
        );
        exit(0)
    }

    fn syntax_error(&self, current: Token, expected: Token) -> ! {
        self.fatal(
            ErrorCode::STATUS_SYNTAX_ERROR,
            format!("expected '{:?}' got '{:?}'.", expected, current),
        )
    }

    fn unexpected(&self, token: Token) {
        println!(
            "syntax error: unexpected '{:?}' in {} -> [{}:{}]",
            token, self.filename, self.line, self.space
        );
        exit(1)
    }

    fn check_value(&mut self) -> Value {
        match self.next() {
            Token::String(value) => Value::STRING(value),
            Token::IntValue(value) => Value::I32(value as i32),
            Token::Identifier(value) => Value::REF(value),
            current => {
                println!(
                    "syntax error: expected expression got '{:?}' in [{}] [{}:{}]",
                    current, self.filename, self.line, self.space
                );
                exit(1)
            }
        }
    }
}
