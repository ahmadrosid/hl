// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
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
                if l.ch == '*' {
                    l.read_char();
                }
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
        if self.ch == '#' {
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
                        self.read_char();
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
                            if self.ch == '(' {
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
                            if self.ch == '=' {
                                return Token::CONSTANT(identifier);
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
                                if ch == '=' {
                                    self.position = position - 1;
                                    self.read_position = position;
                                    let mut value = identifier;
                                    value.append(
                                        &mut self.input[start_position..self.position].to_vec(),
                                    );
                                    return Token::CONSTANT(value);
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

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "nil" | "true" | "false" => Ok(Token::CONSTANT(identifier.clone())),
        "addr" | "and" | "as" | "asm" | "bind" | "block" | "break" | "case" | "cast"
        | "concept" | "const" | "continue" | "converter" | "defer" | "discard" | "distinct"
        | "div" | "do" | "elif" | "else" | "echo" | "end" | "enum" | "except" | "export"
        | "finally" | "for" | "from" | "func" | "if" | "import" | "in" | "include"
        | "interface" | "is" | "isnot" | "iterator" | "let" | "macro" | "method" | "mixin"
        | "mod" | "not" | "notin" | "object" | "of" | "or" | "out" | "proc" | "ptr" | "raise"
        | "ref" | "return" | "shl" | "shr" | "static" | "template" | "try" | "tuple" | "type"
        | "using" | "var" | "when" | "while" | "yield" | "xor" | "int" | "int8" | "int16"
        | "int32" | "int64" | "uint" | "uint8" | "uint16" | "uint32" | "uint64" | "float"
        | "float32" | "float64" | "char" | "string" | "cstring" | "bool" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}

pub fn render_html(input: Vec<char>) -> String {
    let mut l = Lexer::new(input);
    l.read_char();
    let mut html = String::new();
    let mut line = 1;
    html.push_str("<table class=\"highlight-table\">\n");
    html.push_str("<tbody>\n");
    html.push_str("<tr>");
    html.push_str(&format!(
        "<td class=\"hl-num\" data-line=\"{}\"></td><td>",
        line
    ));

    loop {
        let token = l.next_token();
        if token == Token::EOF {
            html.push_str("</td></tr>\n");
            break;
        }

        match token {
            Token::INT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-c\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            Token::IDENT(value) => {
                html.push_str(&value.iter().collect::<String>());
            }
            Token::STRING(value) => {
                let mut s = String::new();
                for ch in value {
                    if ch == '<' {
                        s.push_str("&lt;");
                    } else if ch == '>' {
                        s.push_str("&gt;");
                    } else {
                        s.push(ch);
                    }
                }
                let split = s.split("\n");
                let split_len = split.clone().collect::<Vec<&str>>().len();
                let mut index = 0;
                for val in split {
                    html.push_str(&format!("<span class=\"hl-s\">{}</span>", val));
                    index = index + 1;
                    if index != split_len {
                        line = line + 1;
                        html.push_str("</td></tr>\n");
                        html.push_str(&format!(
                            "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                            line
                        ));
                    }
                }
            }
            Token::ENTITY(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-en\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            Token::CONSTANT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-c\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            Token::KEYWORD(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-k\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            Token::COMMENT(value) => {
                let mut lines = String::new();
                for ch in value {
                    if ch == '<' {
                        lines.push_str("&lt;");
                    } else if ch == '>' {
                        lines.push_str("&gt;");
                    } else {
                        lines.push(ch);
                    }
                }
                let split = lines.split("\n");
                let split_len = split.clone().collect::<Vec<&str>>().len();
                let mut index = 0;
                for val in split {
                    if val.len() > 1 {
                        html.push_str(&format!("<span class=\"hl-cmt\">{}</span>", val));
                    }
                    index = index + 1;
                    if index != split_len {
                        line = line + 1;
                        html.push_str("</td></tr>\n");
                        html.push_str(&format!(
                            "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                            line
                        ));
                    }
                }
            }
            Token::ENDL(_) => {
                line = line + 1;
                html.push_str("</td></tr>\n");
                html.push_str(&format!(
                    "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                    line
                ));
            }
            _ => {
                html.push(l.ch);
                l.read_char();
            }
        }
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>");
    html
}
