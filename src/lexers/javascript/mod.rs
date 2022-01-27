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

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_white_space(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\t' || ch == '\n'
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
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_slash_star_comment = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() {
                if l.position == l.input.len() {
                    break;
                }
                if l.input[l.position + 1] == '*' {
                    if l.input[l.position + 2] == '/' {
                        l.read_char();
                        l.read_char();
                        break;
                    }
                }
                l.read_char();
            }
            l.input[position..l.position + 1].to_vec()
        };

        let tok: token::Token;
        if self.read_position < self.input.len()
            && self.ch == '/'
            && self.input[self.read_position] == '/'
        {
            return token::Token::COMMENT(read_string(self, '\n'));
        }

        match self.ch {
            '\n' => {
                tok = token::Token::ENDL(self.ch);
            }
            '\0' => {
                tok = token::Token::EOF;
            }
            '0' => {
                return if self.input[self.read_position] == 'x' {
                    let start_position = self.position;
                    self.read_char();
                    self.read_char();
                    while self.position < self.input.len()
                        && (is_digit(self.ch) || is_letter(self.ch))
                    {
                        self.read_char()
                    }
                    let hexadecimal = &self.input[start_position..self.position];
                    token::Token::INT(hexadecimal.to_vec())
                } else {
                    let number = read_number(self);
                    token::Token::INT(number)
                }
            }
            '/' => {
                if self.input[self.position + 1] == '*' {
                    tok = token::Token::COMMENT(read_slash_star_comment(self));
                } else {
                    tok = token::Token::IDENT(vec![self.ch]);
                }
            }
            _ => {
                return if is_letter(self.ch) {
                    #[allow(unused_variables)]
                    let start_position = self.position;
                    #[allow(unused_mut)]
                    let mut identifier: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&identifier) {
                        Ok(keyword_token) => keyword_token,
                        Err(_) => {
                            if is_digit(self.ch) {
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
                                return token::Token::ENTITY(identifier);
                            }
                            if start_position > 0 && self.input[start_position - 1] == '.' {
                                return token::Token::ENTITY(identifier);
                            }
                            if self.ch == '(' {
                                return token::Token::ENTITY(identifier);
                            } else if is_white_space(self.ch) {
                                let start_position = self.position;
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && is_white_space(ch) {
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
                                    return token::Token::ENTITY(value);
                                }
                            }
                            if self.ch == ':' {
                                return token::Token::ENTITY(identifier);
                            } else if is_white_space(self.ch) {
                                let start_position = self.position;
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && is_white_space(ch) {
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
                                    return token::Token::ENTITY(value);
                                }
                            }
                            token::Token::IDENT(identifier)
                        }
                    }
                } else if is_digit(self.ch) {
                    let identifier: Vec<char> = read_number(self);
                    token::Token::INT(identifier)
                } else if self.ch == '\'' {
                    let str_value: Vec<char> = read_string(self, '\'');
                    token::Token::STRING(str_value)
                } else if self.ch == '"' {
                    let str_value: Vec<char> = read_string(self, '"');
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
