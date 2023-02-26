use self::{errors::ErrorCode, statament::Stantament, values::Value};
use crate::lexer::tokens::Keywords;
use logos::Lexer;
use std::process::exit;
use colored::Colorize;

pub mod errors;
pub mod statament;
pub mod values;

pub struct Parse {
    filename: String,
    tokens: Vec<Keywords>,
    idx: i32,
    line: usize,
}

impl Parse {
    pub fn new(filename: String, lexer: Lexer<Keywords>) -> Self {
        let mut tokens = Vec::new();
        for token in lexer {
            tokens.push(token);
        }

        Self {
            filename,
            tokens,
            idx: 0,
            line: 1,
        }
    }

    pub fn analyse(&mut self) -> Vec<Stantament> {
        let mut stantament = Vec::new();
        loop {
            match self.next() {
                Keywords::Func => stantament.push(Stantament::create_function(self)),
                Keywords::EOF => break,
                tk => self.unexpected(tk),
            }
        }
        return stantament;
    }

    fn next(&mut self) -> Keywords {
        loop {
            if self.idx >= self.tokens.len() as i32 {
                return Keywords::EOF;
            }

            let current = self.tokens[self.idx as usize].clone();
            match current {
                Keywords::LINE => {
                    self.line += 1;
                    self.idx += 1;
                },
                Keywords::SPACE =>  self.idx += 1,
                Keywords::Error => self.idx += 1,
                _ => {
                    self.idx += 1;
                    break current;
                }
            }
        }
    }

    fn back(&mut self) -> Keywords {
        loop {
            if self.idx - 1 > 0 {
                let current = self.tokens[(self.idx - 2) as usize].clone();
                if current == Keywords::LINE {
                    self.line -= 1;
                    self.idx -= 1;
                } else if current == Keywords::SPACE {
                    self.idx -= 1;
                } else if current == Keywords::Error {
                    self.idx -= 1;
                } else {
                    self.idx -= 1;
                    break current;
                }
            } else {
                break self.tokens[(self.idx - 1) as usize].clone();
            }
        }
    }

    fn check(&mut self, expected: Keywords) {
        let current = self.next();
        if current != expected {
            self.syntax_error(current, expected);
        }
    }

    fn report(&self, status: ErrorCode, note: String) -> ! {
        println!(
            "{}:{} [ status: {:?}, code: {}{}, note: {} ]",
            self.filename,
            self.line.to_string().bright_white(),
            status,
            "E0".bright_red(),
            status.code().to_string().bright_red(),
            note.cyan().bold()
        );
        exit(0)
    }

    fn syntax_error(&self, current: Keywords, expected: Keywords) -> ! {
        self.report(
            ErrorCode::STATUS_SYNTAX_ERROR,
            format!("expected `{:?}`, found `{:?}`.", expected, current),
        )
    }

    fn unexpected(&self, token: Keywords) -> ! {
        self.report(
            ErrorCode::STATUS_SYNTAX_ERROR,
            format!("unexpected token `{:?}`", token)
        );
    }

    fn check_value(&mut self) -> Value {
        match self.next() {
            Keywords::String(value) => Value::STRING(value),
            Keywords::IntValue(value) => Value::I32(value as i32),
            Keywords::Identifier(value) => Value::VAR(value),
            _ => {
                let back = self.back();
                self.report(ErrorCode::STATUS_MISSING_VALUE, format!("missing value after `{:?}`", back));
            }
        }
    }
}
