use logos::Logos;
use scanner::Token;

pub mod scanner;
pub mod types;

#[cfg(test)]
mod tests;

pub struct Lexer {
    filename: String,
    source: String,
    data: Vec<Token>,
    line: usize,
    cursor: usize,
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
            filename: filename.to_string(),
            source: source.to_string(),
            data: buff,
            line: 1,
            cursor: 0,
            idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.line = 1;
        self.idx = 0;
    }

    pub fn next(&mut self) -> Token {
        while self.idx < self.data.len() as i32 {
            let token = &self.data[self.idx as usize];
            self.idx += 1;
            self.cursor += token.to_string().len();
            match token {
                Token::Line => {
                    self.line += 1;
                    self.cursor = 0;
                    continue;
                }
                Token::Space | Token::Error => continue,
                _ => return token.clone(),
            }
        }
        Token::EOF
    }

    pub fn column(&self) -> usize {
        self.cursor
    }

    pub fn peek(&mut self) -> Token {
        let peek = self.next();
        self.back();
        return peek;
    }

    pub fn skip(&mut self) {
        if self.idx >= self.data.len() as i32 {
            return;
        }
        self.idx += 1;
    }

    pub fn skip_line(&mut self) {
        let line = self.line;
        while self.line == line {
            self.next();
        }
        self.back();
    }

    pub fn next_identifier(&mut self) -> Option<String> {
        match self.next() {
            scanner::Token::Identifier(name) => Some(name.to_owned()),
            _ => None,
        }
    }

    pub fn back(&mut self) {
        while self.idx - 1 > 0 {
            match self.data[(self.idx - 1) as usize].clone() {
                Token::Line => {
                    self.line -= 1;
                    self.idx -= 1;
                    self.cursor = 0;
                }
                Token::Space | Token::Error => {
                    self.idx -= 1;
                    self.cursor -= 1
                }
                token => {
                    self.cursor -= token.to_string().len();
                    self.idx -= 1;
                    return;
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

    pub fn get_source_line(&self, line_number: usize) -> Option<&str> {
        let mut current_line = 1;
        let mut start_idx = 0;
        let mut end_idx = 0;
        for (i, c) in self.source.char_indices() {
            if current_line == line_number && (i == 0 || &self.source[i - 1..i] == "\n") {
                start_idx = i;
            }
            if current_line == line_number && c == '\n' {
                end_idx = i;
                break;
            }
            if c == '\n' {
                current_line += 1;
            }
        }
        if end_idx == 0 && current_line == line_number {
            end_idx = self.source.len();
        }
        if start_idx == 0 && end_idx == 0 {
            return None;
        }
        Some(&self.source[start_idx..end_idx])
    }

    pub fn source(&self) -> &str {
        self.source.as_ref()
    }
}
