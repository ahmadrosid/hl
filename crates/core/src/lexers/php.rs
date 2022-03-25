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
            '<' => {
                if self.input[self.position + 1] == '?' {
                    let mut entity = vec!['<', '?'];
                    self.read_char();
                    self.read_char();
                    entity.append(&mut read_identifier(self));
                    return Token::ENTITYTAG(entity);
                } else {
                    tok = Token::CH(self.ch);
                }
            }
            '?' => {
                if self.input[self.position + 1] == '>' {
                    let entity = vec!['?', '>'];
                    self.read_char();
                    self.read_char();
                    return Token::ENTITYTAG(entity);
                } else {
                    tok = Token::CH(self.ch);
                }
            }
            '$' => {
                if is_letter(self.input[self.position + 1]) {
                    let position = self.position;
                    self.read_char();
                    let mut identifier = vec!['$'];
                    identifier.append(&mut read_identifier(self));
                    if self.input[position - 1] == '[' {
                        return Token::CONSTANT(identifier);
                    } else {
                        return Token::VAR(identifier);
                    }
                }
                tok = Token::CH(self.ch);
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
                        Ok(keyword_token) => {
                            if self.ch == '=' && identifier.iter().collect::<String>() == "class" {
                                return Token::CONSTANT(identifier);
                            }
                            keyword_token
                        }
                        Err(_) => {
                            if self.ch == '-' {
                                let last_position = self.position;
                                self.read_char();
                                while self.position < self.input.len() && is_letter(self.ch) {
                                    self.read_char();
                                }
                                identifier
                                    .append(&mut self.input[last_position..self.position].to_vec());
                            }
                            if self.ch == '=' {
                                return Token::CONSTANT(identifier);
                            }
                            if start_position > 0 && self.input[start_position - 1] == '>' {
                                return Token::ENTITY(identifier);
                            }
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
        "true" | "false" | "null" | "string" | "array" => Ok(Token::CONSTANT(identifier.clone())),
        "abstract" | "as" | "break" | "and" | "callable" | "case" | "catch" | "clone" | "const"
        | "continue" | "declare" | "do" | "echo" | "else" | "enddeclare" | "endfor"
        | "endforeach" | "endswitch" | "endwhile" | "eval" | "finally" | "for" | "foreach"
        | "global" | "goto" | "if" | "implements" | "include_once" | "instanceof" | "insteadof"
        | "interface" | "namespace" | "new" | "or" | "private" | "protected" | "public"
        | "require" | "require_once" | "return" | "static" | "switch" | "throw" | "try" | "use"
        | "class" | "default" | "elseif" | "endif" | "extends" | "function" | "include"
        | "print" | "trait" | "while" | "xor" | "yield" => Ok(Token::KEYWORD(identifier.clone())),
        "html" | "body" | "div" | "span" | "applet" | "object" | "iframe" | "h1" | "h2" | "h3"
        | "h4" | "h5" | "h6" | "p" | "blockquote" | "button" | "pre" | "a" | "abbr" | "acronym"
        | "address" | "big" | "cite" | "code" | "del" | "dfn" | "em" | "img" | "ins" | "kbd"
        | "q" | "s" | "samp" | "select" | "small" | "strike" | "strong" | "sub" | "sup"
        | "script" | "tt" | "var" | "b" | "u" | "i" | "center" | "dl" | "dt" | "dd" | "ol"
        | "ul" | "li" | "fieldset" | "form" | "label" | "legend" | "table" | "caption"
        | "tbody" | "tfoot" | "thead" | "tr" | "th" | "td" | "article" | "canvas" | "embed"
        | "output" | "ruby" | "summary" | "time" | "mark" | "audio" | "videoarticle" | "aside"
        | "details" | "figcaption" | "figure" | "footer" | "header" | "hgroup" | "menu" | "nav"
        | "section" | "video" | "textarea" | "input" | "hr" => {
            Ok(Token::ENTITYTAG(identifier.clone()))
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
                html.push_str(&format!("<span class=\"hl-s\">{}</span>", s));
            }
            Token::CH(value) => {
                if value == '<' {
                    html.push_str("&lt;");
                } else {
                    html.push(value);
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
            Token::ENTITYTAG(value) => {
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
                html.push_str(&format!("<span class=\"hl-ent\">{}</span>", s));
            }
            Token::VAR(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-vid\">{}</span>",
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
                if l.ch == '<' {
                    html.push_str("&lt;");
                    l.read_char();
                    continue;
                }
                html.push(l.ch);
                l.read_char();
            }
        }
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>");
    html
}
