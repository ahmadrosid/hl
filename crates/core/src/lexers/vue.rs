// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
    template_depth: usize,
    in_script: bool,
    in_style: bool,
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn vue_template_plain_text(l: &Lexer, start_position: usize) -> bool {
    l.template_depth > 0
        && !l.in_script
        && !l.in_style
        && (start_position == 0 || {
            let prev = l.input[start_position - 1];
            prev != '<' && prev != '/' && prev != ':' && prev != '#' && prev != '@' && prev != '.'
        })
}
impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            template_depth: 0,
            in_script: false,
            in_style: false,
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
                if l.ch == '?' {
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
        if self.ch == '<' {
            let next_id = String::from("<!--").chars().collect::<Vec<_>>();
            let next_position = self.position + next_id.len();
            let end_id = String::from("-->").chars().collect::<Vec<_>>();
            if self.position + next_id.len() < self.input.len()
                && self.input[self.position..next_position] == next_id
            {
                let mut identifier = next_id.clone();
                next_id.iter().for_each(|_| self.read_char());
                let start_position = self.position;
                while self.position < self.input.len() {
                    if self.ch == '-' {
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
                        self.read_char();
                    }
                    let hexadecimal = &self.input[start_position..self.position];
                    Token::INT(hexadecimal.to_vec())
                } else {
                    let number = read_number(self);
                    Token::INT(number)
                }
            }
            '#' => {
                if is_letter(self.input[self.position + 1]) {
                    let mut identifier = vec![self.ch];
                    self.read_char();
                    identifier.append(&mut read_identifier(self));
                    return Token::KEYWORD(identifier);
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
                            let position = if start_position == 0 {
                                0
                            } else {
                                start_position - 1
                            };
                            if self.input[position] == '.' {
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                return Token::ENTITY(identifier);
                            }
                            if start_position > 0 {
                                let prev = self.input[start_position - 1];
                                let sfc_tag: String = identifier.iter().collect();
                                if prev == '<' {
                                    if sfc_tag == "template" {
                                        self.template_depth += 1;
                                    } else if sfc_tag == "script" {
                                        self.in_script = true;
                                    } else if sfc_tag == "style" {
                                        self.in_style = true;
                                    }
                                } else if prev == '/' {
                                    if sfc_tag == "template" {
                                        self.template_depth = self.template_depth.saturating_sub(1);
                                    } else if sfc_tag == "script" {
                                        self.in_script = false;
                                    } else if sfc_tag == "style" {
                                        self.in_style = false;
                                    }
                                }
                            }
                            if vue_template_plain_text(self, start_position) {
                                return Token::IDENT(identifier);
                            }
                            keyword_token
                        }
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
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                return Token::ENTITY(identifier);
                            }
                            if self.ch == '-' {
                                let last_position = self.position;
                                self.read_char();
                                while self.position < self.input.len() && is_letter(self.ch) {
                                    self.read_char();
                                }
                                identifier
                                    .append(&mut self.input[last_position..self.position].to_vec());
                            }
                            if start_position > 0 && self.input[start_position - 1] == '.' {
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                return Token::ENTITY(identifier);
                            }
                            if self.ch == '(' {
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                return Token::ENTITY(identifier);
                            } else if self.ch.is_whitespace() {
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && ch.is_whitespace() {
                                    position += 1;
                                    if position < self.input.len() {
                                        ch = self.input[position];
                                    }
                                }
                                if ch == '(' {
                                    if vue_template_plain_text(self, start_position) {
                                        return Token::IDENT(identifier);
                                    }
                                    return Token::ENTITY(identifier);
                                }
                            }
                            if self.ch == ':' {
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                if vue_template_plain_text(self, start_position) {
                                    return Token::IDENT(identifier);
                                }
                                return Token::ENTITY(identifier);
                            } else if self.ch.is_whitespace() {
                                let mut position = self.position;
                                let mut ch = self.input[position];
                                while position < self.input.len() && ch.is_whitespace() {
                                    position += 1;
                                    if position < self.input.len() {
                                        ch = self.input[position];
                                    }
                                }
                                if ch == ':' {
                                    if vue_template_plain_text(self, start_position) {
                                        return Token::IDENT(identifier);
                                    }
                                    return Token::ENTITY(identifier);
                                }
                            }
                            if start_position > 0 {
                                let prev = self.input[start_position - 1];
                                if prev == '<' || prev == '/' {
                                    if identifier
                                        .first()
                                        .map(|c| c.is_uppercase())
                                        .unwrap_or(false)
                                    {
                                        return Token::ENTITYTAG(identifier);
                                    }
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
        "true" | "false" | "undefined" | "null" | "number" | "string" | "boolean" | "root"
        | "rgb" | "rgba" | "calc" | "media" => Ok(Token::CONSTANT(identifier.clone())),
        "Infinity" | "NaN" | "Math" | "Date" => Ok(Token::VAR(identifier.clone())),
        "after" | "before" | "hover" | "not" | "focus" | "active" | "selection" | "px" | "rem" => {
            Ok(Token::ENTITY(identifier.clone()))
        }
        "async" | "await" | "break" | "as" | "switch" | "case" | "if" | "throw" | "else"
        | "var" | "get" | "module" | "instanceof" | "import" | "from" | "typeof" | "public"
        | "private" | "enum" | "export" | "finally" | "for" | "while" | "void" | "super"
        | "this" | "in" | "return" | "any" | "extends" | "static" | "let" | "package"
        | "implements" | "interface" | "function" | "new" | "try" | "yield" | "const"
        | "continue" | "do" | "catch" | "default" | "class" | "debugger" | "delete"
        | "protected" | "with" | "readonly" | "declare" | "namespace" | "keyof" | "infer"
        | "type" | "important" | "v-if" | "v-else" | "v-else-if" | "v-for" | "v-show"
        | "v-model" | "v-bind" | "v-on" | "v-slot" | "v-pre" | "v-once" | "v-memo" | "v-html"
        | "v-text" | "defineProps" | "defineEmits" | "defineExpose" | "defineModel"
        | "withDefaults" | "ref" | "reactive" | "computed" | "watch" | "onMounted"
        | "onUnmounted" => Ok(Token::KEYWORD(identifier.clone())),
        "template" | "style" | "html" | "head" | "title" | "meta" | "link" | "body" | "div"
        | "span" | "applet" | "object" | "iframe" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
        | "p" | "blockquote" | "button" | "pre" | "a" | "abbr" | "acronym" | "address" | "big"
        | "cite" | "code" | "del" | "dfn" | "em" | "img" | "ins" | "kbd" | "q" | "s" | "samp"
        | "select" | "small" | "strike" | "strong" | "sub" | "sup" | "script" | "tt" | "b"
        | "u" | "i" | "center" | "dl" | "dt" | "dd" | "ol" | "ul" | "li" | "fieldset" | "form"
        | "label" | "legend" | "table" | "caption" | "tbody" | "tfoot" | "thead" | "tr" | "th"
        | "td" | "article" | "canvas" | "embed" | "output" | "ruby" | "summary" | "time"
        | "mark" | "audio" | "videoarticle" | "aside" | "details" | "figcaption" | "figure"
        | "footer" | "header" | "hgroup" | "menu" | "nav" | "section" | "video" | "textarea"
        | "input" | "hr" => Ok(Token::ENTITYTAG(identifier.clone())),
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
            Token::VAR(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-v\">{}</span>",
                    value.iter().collect::<String>()
                ));
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
                if l.ch == '>' {
                    html.push_str("&gt;");
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
