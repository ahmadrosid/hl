pub mod token;
pub mod render;

pub struct Lexer {
  input: Vec<char>,
  pub position: usize,
  pub read_position: usize,
  pub ch: char
}

fn is_letter(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
  '0' <= ch && ch <= '9'
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Self {
    Self {
        input,
        position: 0,
        read_position: 0,
        ch: '0'
    }
}

pub fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
        self.ch = '0';
    } else {
        self.ch = self.input[self.read_position];
    }
    self.position = self.read_position;
    self.read_position = self.read_position + 1;
}

pub fn skip_whitespace(&mut self) {
    let ch = self.ch;
    if ch == '\r' {
        self.read_char();
    }
}

pub fn next_token(&mut self) -> token::Token {
    let read_identifier = |l: &mut Lexer| -> Vec<char> {
        let position = l.position;
        while l.position < l.input.len() && is_letter(l.ch) {
            l.read_char();
        }
        l.input[position..l.position].to_vec()
    };

    let read_string = |l: &mut Lexer| -> Vec<char> {
        let position = l.position;
        l.read_char();
        while l.position < l.input.len() && l.ch != '"' {
            l.read_char()
        }
        l.read_char();
        l.input[position..l.position].to_vec()
    };

    let read_number = |l: &mut Lexer| -> Vec<char> {
        let position = l.position;
        while l.position < l.input.len() && is_digit(l.ch) {
            l.read_char();
        }
        l.input[position..l.position].to_vec()
    };

    let tok: token::Token;
    self.skip_whitespace();
    match self.ch {
        '=' => {
            tok = token::Token::ASSIGN(self.ch);
        },
        '+' => {
            tok = token::Token::PLUS(self.ch);
        },
        '-' => {
            tok = token::Token::MINUS(self.ch);
        },
        '!' => {
            tok = token::Token::BANG(self.ch);
        },
        '/' => {
            tok = token::Token::SLASH(self.ch);
        },
        '*' => {
            tok = token::Token::ASTERISK(self.ch);
        },
        '<' => {
            tok = token::Token::LT(self.ch);
        },
        '>' => {
            tok = token::Token::GT(self.ch);
        },
        ';' => {
            tok = token::Token::SEMICOLON(self.ch);
        },
        ':' => {
            let next_ch = self.input[self.position+1];
            if next_ch == ':' {
                self.read_char();
                tok = token::Token::PATHSEPARATOR(self.input[self.position-1..self.position+1].to_vec());
            } else {
                tok = token::Token::COLON(self.ch);
            }
        },
        '.' => {
            tok = token::Token::MEMBERACCESS(self.ch);
        },
        '(' => {
            tok = token::Token::LPAREN(self.ch);
        },
        ')' => {
            tok = token::Token::RPAREN(self.ch);
        },
        ',' => {
            tok = token::Token::COMMA(self.ch);
        },
        '{' => {
            tok = token::Token::LBRACE(self.ch);
        },
        '}' => {
            tok = token::Token::RBRACE(self.ch);
        },
        ' ' => {
            tok = token::Token::SPACE;
        }
        '\n' => {
            tok = token::Token::ENL;
        }
        '\t' => {
            tok = token::Token::TAB;
        }
        '0' => {
            tok = token::Token::EOF;
        }
        _ => {
            return if is_letter(self.ch) {
                let identifier: Vec<char> = read_identifier(self);
                match token::get_keyword_token(&identifier) {
                    Ok(keyword_token) => {
                        keyword_token
                    },
                    Err(_err) => {
                        token::Token::IDENT(identifier)
                    }
                }
            } else if is_digit(self.ch) {
                let identifier: Vec<char> = read_number(self);
                token::Token::INT(identifier)
            } else if self.ch == '"' {
                let str_value: Vec<char> = read_string(self);
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
