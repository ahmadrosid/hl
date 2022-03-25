// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
pub mod render;

use crate::lexers::Token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
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
        self.read_position += 1;
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
                    l.read_char();
                }
                l.read_char();
            }
            l.read_char();
            if l.position > l.input.len() {
                l.position -= 1;
                l.read_position -= 1;
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
        if self.ch == '{' {
            let next_ch = self.input[self.position + 1];
            if self.position + 1 < self.input.len() && next_ch == '-' {
                let mut comment = vec!['{', '-'];
                self.read_char();
                self.read_char();
                let last_position = self.position;
                while self.position < self.input.len() {
                    if self.read_position <= self.input.len()
                        && self.ch == '-'
                        && self.input[self.read_position] == '}'
                    {
                        break;
                    }
                    self.read_char();
                }
                self.read_char();
                self.read_char();
                comment.append(&mut self.input[last_position..self.position].to_vec());
                return Token::COMMENT(comment);
            }
        }
        if self.ch == '`' {
            return Token::KEYWORD(read_string(self, '`'));
        }
        if self.read_position < self.input.len()
            && self.ch == '-'
            && self.input[self.read_position] == '-'
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
            _ => {
                return if is_letter(self.ch) {
                    #[allow(unused_variables)]
                    let start_position = self.position;
                    #[allow(unused_mut)]
                    let mut identifier: Vec<char> = read_identifier(self);
                    if self.ch.is_numeric() {
                        let position = self.position;
                        while self.position < self.input.len() {
                            if !self.ch.is_numeric() && !is_letter(self.ch) {
                                break;
                            }
                            self.read_char();
                        }
                        identifier.append(&mut self.input[position..self.position].to_vec());
                    }
                    match get_keyword_token(&identifier) {
                        Ok(keyword_token) => keyword_token,
                        Err(_) => {
                            if start_position > 0 && self.input[start_position - 1] == ':' {
                                return Token::ENTITY(identifier);
                            }
                            if self.ch == ':' {
                                return Token::ENTITY(identifier);
                            } else if self.ch.is_whitespace() {
                                let start_position = self.position;
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && ch.is_whitespace() {
                                    position += 1;
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

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "True" | "False" | "return" | "mempty" | "not" | "foldr" | "Map" | "lookup" | "Left"
        | "Right" | "Walk" | "Char" | "Bool" | "Maybe" | "Int" | "Prelude" | "any" | "filter"
        | "map" | "init" | "null" | "last" | "unwords" | "otherwise" | "either" | "const"
        | "fmap" | "take" | "max" | "putStrLn" => Ok(Token::CONSTANT(identifier.clone())),
        "as" | "import" | "type" | "case" | "do" | "of" | "if" | "then" | "else" | "let" | "in"
        | "module" | "qualified" | "where" | "LANGUAGE" | "OPTIONS_GHC" | "hiding" | "data"
        | "deriving" => Ok(Token::KEYWORD(identifier.clone())),
        "Bibtex" | "Biblatex" | "Nothing" | "Just" | "Str" | "Space" | "FancyVal" | "BibState"
        | "Many" => Ok(Token::ENTITYTAG(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
