// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
pub mod render;
mod token;

use crate::lexers::Token;
use token::get_keyword_token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_string = |l: &mut Lexer, ch: char| -> Vec<char> {
            let position = l.position;
            l.read_char();
            while l.position < l.input.len() && l.ch != ch {
                if l.ch == '\\' {
                    l.read_char()
                }
                l.read_char();
            }
            l.read_char();
            if l.position > l.input.len() {
                l.position = l.position - 1;
                l.read_position = l.read_position - 1;
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_numeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: Token;
        if self.ch == '/' {
            let next_id = String::from("/*").chars().collect::<Vec<_>>();
            let next_position = self.position + next_id.len();
            let end_id = String::from("*/").chars().collect::<Vec<_>>();
            if self.position + next_id.len() < self.input.len()
                && self.input[self.position..next_position] == next_id
            {
                let mut identifier = next_id.clone();
                next_id.iter().for_each(|_| self.read_char());
                let start_position = self.position;
                while self.position < self.input.len() {
                    if self.ch == '*' {
                        let end_position = self.position + end_id.len();
                        if end_position <= self.input.len()
                            && self.input[self.position..end_position] == end_id
                        {
                            end_id.to_owned().iter().for_each(|_| self.read_char());
                            break;
                        }
                    }
                    self.read_char();
                }
                identifier.append(&mut self.input[start_position..self.position].to_vec());
                return Token::COMMENT(identifier);
            }
        }
        if self.ch == '`' {
            let next_id = String::from("`").chars().collect::<Vec<_>>();
            let next_position = self.position + next_id.len();
            let end_id = String::from("`").chars().collect::<Vec<_>>();
            if self.position + next_id.len() < self.input.len()
                && self.input[self.position..next_position] == next_id
            {
                let mut identifier = next_id.clone();
                next_id.iter().for_each(|_| self.read_char());
                let start_position = self.position;
                while self.position < self.input.len() {
                    if self.ch == '`' {
                        let end_position = self.position + end_id.len();
                        if end_position <= self.input.len()
                            && self.input[self.position..end_position] == end_id
                        {
                            end_id.to_owned().iter().for_each(|_| self.read_char());
                            break;
                        }
                    }
                    self.read_char();
                }
                identifier.append(&mut self.input[start_position..self.position].to_vec());
                return Token::STRING(identifier);
            }
        }
        if self.read_position < self.input.len()
            && self.ch == '/'
            && self.input[self.read_position] == '/'
        {
            return Token::COMMENT(read_string(self, '\n'));
        }

        match self.ch {
            '\n' => {
                tok = Token::ENDL(self.ch);
            }
            '\0' => {
                tok = Token::EOF;
            }
            '0' => {
                return if self.input[self.read_position] == 'x' {
                    let start_position = self.position;
                    self.read_char();
                    self.read_char();
                    while self.position < self.input.len()
                        && (self.ch.is_numeric() || is_letter(self.ch))
                    {
                        self.read_char()
                    }
                    let hexadecimal = &self.input[start_position..self.position];
                    Token::INT(hexadecimal.to_vec())
                } else {
                    let number = read_number(self);
                    Token::INT(number)
                }
            }
            _ => {
                return if is_letter(self.ch) {
                    #[allow(unused_variables)]
                    let start_position = self.position;
                    #[allow(unused_mut)]
                    let mut identifier: Vec<char> = read_identifier(self);
                    match get_keyword_token(&identifier) {
                        Ok(keyword_token) => keyword_token,
                        Err(_) => {
                            if self.ch.is_numeric() {
                                let position = self.position;
                                while self.position < self.input.len() {
                                    if self.ch == ' '
                                        || self.ch == ':'
                                        || self.ch == ':'
                                        || self.ch == '('
                                        || self.ch == '{'
                                        || self.ch == '\n'
                                    {
                                        break;
                                    }
                                    self.read_char();
                                }
                                identifier
                                    .append(&mut self.input[position..self.position].to_vec());
                                return Token::ENTITY(identifier);
                            }
                            if start_position > 0 && self.input[start_position - 1] == '.' {
                                return Token::ENTITY(identifier);
                            }
                            if self.ch == '(' {
                                return Token::ENTITY(identifier);
                            } else if self.ch.is_whitespace() {
                                let start_position = self.position;
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && ch.is_whitespace() {
                                    position = position + 1;
                                    if position < self.input.len() {
                                        ch = self.input[position];
                                    }
                                }
                                if ch == '(' {
                                    self.position = position - 1;
                                    self.read_position = position;
                                    let mut value = identifier;
                                    value.append(
                                        &mut self.input[start_position..self.position].to_vec(),
                                    );
                                    return Token::ENTITY(value);
                                }
                            }
                            if self.ch == ':' {
                                return Token::ENTITY(identifier);
                            } else if self.ch.is_whitespace() {
                                let start_position = self.position;
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && ch.is_whitespace() {
                                    position = position + 1;
                                    if position < self.input.len() {
                                        ch = self.input[position];
                                    }
                                }
                                if ch == ':' {
                                    self.position = position - 1;
                                    self.read_position = position;
                                    let mut value = identifier;
                                    value.append(
                                        &mut self.input[start_position..self.position].to_vec(),
                                    );
                                    return Token::ENTITY(value);
                                }
                            }
                            Token::IDENT(identifier)
                        }
                    }
                } else if self.ch.is_numeric() {
                    let identifier: Vec<char> = read_number(self);
                    Token::INT(identifier)
                } else if self.ch == '\'' {
                    let str_value: Vec<char> = read_string(self, '\'');
                    Token::STRING(str_value)
                } else if self.ch == '"' {
                    let str_value: Vec<char> = read_string(self, '"');
                    Token::STRING(str_value)
                } else {
                    Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }
}