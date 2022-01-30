// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
pub mod render;
pub mod token;

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

    pub fn next_token(&mut self) -> token::Token {
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

        let tok: token::Token;
        match self.ch {
            '\n' => {
                tok = token::Token::ENDL(self.ch);
            }
            '\0' => {
                tok = token::Token::EOF;
            }
            _ => {
                return if is_letter(self.ch) {
                    #[allow(unused_variables)]
                    let start_position = self.position;
                    #[allow(unused_mut)]
                    let mut identifier: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&identifier) {
                        Ok(keyword_token) => keyword_token,
                        Err(_) => token::Token::IDENT(identifier),
                    }
                } else if self.ch.is_numeric() {
                    let identifier: Vec<char> = read_number(self);
                    token::Token::INT(identifier)
                } else if self.ch == '"' {
                    let str_value: Vec<char> = read_string(self, '"');
                    if self.ch == ':' {
                        return token::Token::ENTITYTAG(str_value);
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
                            self.position = position;
                            self.read_position = position + 1;
                            let mut value = str_value;
                            value.append(
                                &mut self.input[start_position..self.read_position].to_vec(),
                            );
                            return token::Token::ENTITYTAG(value);
                        }
                    }
                    token::Token::STRING(str_value)
                } else {
                    token::Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }
}
