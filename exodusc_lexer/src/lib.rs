use scanner::{Token};
use logos::Logos;

pub mod expressions;
pub mod scanner;
pub mod types;
pub mod values;

#[cfg(test)]
mod tests;

pub struct Lexer {
    filename: String,
    data: Vec<Token>,
    line: usize,
    idx: i32,
}

impl Lexer {
    pub fn tokenization(filename: &str, source: &str) -> Self {
        let tokens = scanner::Token::lexer(&source);
        let mut buff = Vec::new();
        for token in tokens {
            buff.push(token);
        }
        Self {
            data: buff,
            filename: filename.to_string(),
            line: 1,
            idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.line = 1;
        self.idx = 0;
    }

    pub fn next(&mut self) -> Token {
        loop {
            if self.idx >= self.data.len() as i32 {
                return Token::EOF;
            }

            match self.data[self.idx as usize].clone() {
                Token::Line => {
                    self.line += 1;
                    self.idx += 1;
                }
                Token::Space => self.idx += 1,
                Token::Error => self.idx += 1,
                token => {
                    self.idx += 1;
                    return token;
                }
            }
        }
    }

    pub fn back(&mut self) -> Token {
        loop {
            if self.idx - 1 <= 0 {
                return Token::EOF;
            }

            match self.data[(self.idx - 2) as usize].clone() {
                Token::Line => {
                    self.line -= 1;
                    self.idx -= 1;
                },
                Token::Space => self.idx -= 1,
                Token::Error => self.idx -= 1,
                tk => {
                    self.idx -= 1;
                    return tk;
                }
            }
        }
    }

    pub fn filename(&self) -> &str {
        self.filename.as_ref()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}