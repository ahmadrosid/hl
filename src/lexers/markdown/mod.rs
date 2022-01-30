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

        if self.position > 1
            && self.position < self.input.len()
            && self.input[self.position] == '#'
            && self.input[self.position - 1] == '\n'
            || (self.position == 0 && self.input[self.position] == '#')
        {
            if self.position + 2 < self.input.len()
                && self.input[self.position..self.position + 2] == vec!['#', ' ']
            {
                let start_position = self.position;
                self.position += 2;
                self.read_position += 2;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
            if self.position + 3 < self.input.len()
                && self.input[self.position..self.position + 3] == vec!['#', '#', ' ']
            {
                let start_position = self.position;
                self.position += 3;
                self.read_position += 3;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
            if self.position + 4 < self.input.len()
                && self.input[self.position..self.position + 4] == vec!['#', '#', '#', ' ']
            {
                let start_position = self.position;
                self.position += 4;
                self.read_position += 4;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
            if self.position + 5 < self.input.len()
                && self.input[self.position..self.position + 5] == vec!['#', '#', '#', '#', ' ']
            {
                let start_position = self.position;
                self.position += 5;
                self.read_position += 5;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
            if self.position + 6 < self.input.len()
                && self.input[self.position..self.position + 6]
                    == vec!['#', '#', '#', '#', '#', ' ']
            {
                let start_position = self.position;
                self.position += 6;
                self.read_position += 6;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
            if self.position + 7 < self.input.len()
                && self.input[self.position..self.position + 7]
                    == vec!['#', '#', '#', '#', '#', '#', ' ']
            {
                let start_position = self.position;
                self.position += 7;
                self.read_position += 7;
                self.ch = self.input[self.position];
                let mut start_mark = self.input[start_position..self.position].to_vec();
                while self.position < self.input.len() {
                    start_mark.push(self.ch);
                    self.read_char();
                    if self.ch == '\n' {
                        break;
                    }
                }
                return token::Token::HEAD(start_mark);
            }
        }
        if self.position > 0 && self.input[self.position - 1] == '\n' && self.ch == '>' {
            return token::Token::COMMENT(read_string(self, '\n'));
        }

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
                    token::Token::IDENT(identifier)
                } else {
                    token::Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }
}
