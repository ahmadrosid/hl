pub mod token;

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
        input: input,
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
    if ch == '\t' || ch == '\r' {
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
        '0' => {
            tok = token::Token::EOF;
        }
        _ => {
            if is_letter(self.ch) {
                let ident: Vec<char> = read_identifier(self);
                match token::get_keyword_token(&ident) {
                    Ok(keywork_token) => {
                        return keywork_token;
                    },
                    Err(_err) => {
                        return token::Token::IDENT(ident);
                    }
                }
            } else if is_digit(self.ch) {
                let ident: Vec<char> = read_number(self);
                return token::Token::INT(ident);
            } 
            else {
                return token::Token::ILLEGAL
            }
        }
    }
    self.read_char();
    tok
  }

}
