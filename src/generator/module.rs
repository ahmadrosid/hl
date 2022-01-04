use crate::generator::{
    get_condition, get_entity_prefix, get_entity_suffix, slash_comment_enable,
    slash_star_comment_enable, string::StringBuilder, xml_comment_enable,
};
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const ACCEPT_ENTITY_TAG_SUFFIX: &str = "ACCEPT_ENTITY_TAG_SUFFIX";
const BREAK_ENTITY_TAG_SUFFIX: &str = "BREAK_ENTITY_TAG_SUFFIX";
const ACCEPT_ENTITY_PREFIX: &str = "ACCEPT_ENTITY_PREFIX";
const BREAK_ENTITY_PREFIX: &str = "BREAK_ENTITY_PREFIX";
const ACCEPT_IDENT_SUFFIX: &str = "ACCEPT_IDENT_SUFFIX";
const BREAK_IDENT_SUFFIX: &str = "BREAK_IDENT_SUFFIX";
const ACCEPT_STRING_ONE_QUOTE: &str = "ACCEPT_STRING_ONE_QUOTE";
const ACCEPT_STRING_DOUBLE_QUOTE: &str = "ACCEPT_STRING_DOUBLE_QUOTE";
const ACCEPT_SUFFIX_DIGIT: &str = "ACCEPT_SUFFIX_DIGIT";
const ACCEPT_ENTITY_TAG_PREFIX: &str = "ACCEPT_ENTITY_TAG_PREFIX";
const ENTITY_TAG_PREFIX_CHAR: &str = "ENTITY_TAG_PREFIX_CHAR";
const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ENTITY_CLOSE_TAG_SUFFIX_CHAR: &str = "ENTITY_CLOSE_TAG_SUFFIX_CHAR";
const VAR_CONSTANT_PREFIX: &str = "VAR_CONSTANT_PREFIX";
const CONSTANT_SUFFIX_CHAR: &str = "CONSTANT_SUFFIX_CHAR";
const ACCEPT_CONSTANT_SUFFIX_KEYWORD: &str = "ACCEPT_CONSTANT_SUFFIX_KEYWORD";
const ACCEPT_CONSTANT_SUFFIX_IDENTIFIER: &str = "ACCEPT_CONSTANT_SUFFIX_IDENTIFIER";
const CONSTANT_SUFFIX_KEYWORD: &str = "CONSTANT_SUFFIX_KEYWORD";
const ACCEPT_DASH_IDENTIFIER: &str = "ACCEPT_DASH_IDENTIFIER";

fn write_struct_lexer(module: &mut StringBuilder) {
    module.push_strln("pub struct Lexer {");
    module.push_tabln(1, "input: Vec<char>,");
    module.push_tabln(1, "pub position: usize,");
    module.push_tabln(1, "pub read_position: usize,");
    module.push_tabln(1, "pub ch: char,");
    module.push_strln("}\n");
}

fn write_helper_is_letter(module: &mut StringBuilder) {
    module.push_strln("fn is_letter(ch: char) -> bool {");
    module.push_tabln(
        1,
        "'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'",
    );
    module.push_strln("}\n");
}

fn write_helper_is_digit(module: &mut StringBuilder) {
    module.push_strln("fn is_digit(ch: char) -> bool {");
    module.push_tabln(1, "'0' <= ch && ch <= '9'");
    module.push_strln("}\n");
}

fn write_impl_lexer(module: &mut StringBuilder, h: &Hash) {
    module.push_strln("impl Lexer {");
    module.push_tabln(1, "pub fn new(input: Vec<char>) -> Self {");
    module.push_tabln(2, "Self {");
    module.push_tabln(3, "input,");
    module.push_tabln(3, "position: 0,");
    module.push_tabln(3, "read_position: 0,");
    module.push_tabln(3, "ch: '\\0',");
    module.push_tabln(2, "}");
    module.push_tabln(1, "}\n");

    module.push_tabln(1, "pub fn read_char(&mut self) {");
    module.push_tabln(2, "if self.read_position >= self.input.len() {");
    module.push_tabln(3, "self.ch = '\\0';");
    module.push_tabln(2, "} else {");
    module.push_tabln(3, "self.ch = self.input[self.read_position];");
    module.push_tabln(2, "}");
    module.push_tabln(2, "self.position = self.read_position;");
    module.push_tabln(2, "self.read_position = self.read_position + 1;");
    module.push_tabln(1, "}\n");

    module.push_tabln(1, "pub fn next_token(&mut self) -> token::Token {");
    module.push_tabln(2, "let read_identifier = |l: &mut Lexer| -> Vec<char> {");
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "while l.position < l.input.len() && is_letter(l.ch) {");
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");

    module.push_tabln(
        2,
        "let read_string = |l: &mut Lexer, ch: char| -> Vec<char> {",
    );
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "l.read_char();");
    module.push_tabln(3, "while l.position < l.input.len() && l.ch != ch {");
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.read_char();");
    module.push_tabln(3, "if l.position > l.input.len() {");
    module.push_tabln(4, "l.position = l.position - 1;");
    module.push_tabln(4, "l.read_position = l.read_position - 1;");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");

    module.push_tabln(2, "let read_number = |l: &mut Lexer| -> Vec<char> {");
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "while l.position < l.input.len() && is_digit(l.ch) {");
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");

    if slash_comment_enable(h) {
        module.push_tabln(2, "let read_slash_comment = |l: &mut Lexer| -> Vec<char> {");
        module.push_tabln(3, "let position = l.position;");
        module.push_tabln(3, "while l.position < l.input.len() {");
        module.push_tabln(4, "l.read_char();");
        module.push_tabln(4, "if l.input[l.position + 1] == '\\n' {");
        module.push_tabln(5, "break;");
        module.push_tabln(4, "}");
        module.push_tabln(3, "}");
        module.push_tabln(3, "l.input[position..l.position + 1].to_vec()");
        module.push_tabln(2, "};\n");
    }

    if slash_star_comment_enable(h) {
        module.push_tabln(
            2,
            "let read_slash_star_comment = |l: &mut Lexer| -> Vec<char> {",
        );
        module.push_tabln(3, "let position = l.position;");
        module.push_tabln(3, "while l.position < l.input.len() {");
        module.push_tabln(4, "if l.position == l.input.len() {");
        module.push_tabln(5, "break;");
        module.push_tabln(4, "}");
        module.push_tabln(4, "if l.input[l.position + 1] == '*' {");
        module.push_tabln(5, "if l.input[l.position + 2] == '/' {");
        module.push_tabln(6, "l.read_char();");
        module.push_tabln(6, "l.read_char();");
        module.push_tabln(6, "break;");
        module.push_tabln(5, "}");
        module.push_tabln(4, "}");
        module.push_tabln(4, "l.read_char();");
        module.push_tabln(3, "}");
        module.push_tabln(3, "l.input[position..l.position + 1].to_vec()");
        module.push_tabln(2, "};\n")
    }

    module.push_tabln(2, "let tok: token::Token;");

    if xml_comment_enable(h) {
        module.push_str(&write_handle_xml_comment());
    }

    module.push_tabln(2, "match self.ch {");
    module.push_tabln(3, "'\\n' => {");
    module.push_tabln(4, "tok = token::Token::ENDL(self.ch);");
    module.push_tabln(3, "}");

    module.push_tabln(3, "'\\0' => {");
    module.push_tabln(4, "tok = token::Token::EOF;");
    module.push_tabln(3, "}");

    if let Some(prefix) = get_condition(h).get(&Yaml::String(ACCEPT_ENTITY_TAG_PREFIX.to_string()))
    {
        if let Some(ch) = get_condition(h).get(&Yaml::String(ENTITY_TAG_PREFIX_CHAR.to_string())) {
            module.push_tabln(3, &format!("'{}' => {{", prefix.as_str().unwrap()));
            module.push_tabln(
                4,
                &format!(
                    "if self.input[self.position + 1] == '{}' {{",
                    ch.as_str().unwrap()
                ),
            );
            module.push_tabln(
                5,
                &format!(
                    "let mut entity = vec!['{}','{}'];",
                    prefix.as_str().unwrap(),
                    ch.as_str().unwrap()
                ),
            );
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(5, "entity.append(&mut read_identifier(self));");
            module.push_tabln(5, "return token::Token::ENTITYTAG(entity);");
            module.push_tabln(4, "} else {");
            module.push_tabln(5, "tok = token::Token::CH(self.ch);");
            module.push_tabln(4, "}");
            module.push_tabln(3, "}");
        }
    }

    if let Some(prefix) = get_condition(h).get(&Yaml::String(ENTITY_TAG_PREFIX_CHAR.to_string())) {
        if let Some(ch) =
            get_condition(h).get(&Yaml::String(ENTITY_CLOSE_TAG_SUFFIX_CHAR.to_string()))
        {
            module.push_tabln(3, &format!("'{}' => {{", prefix.as_str().unwrap()));
            module.push_tabln(
                4,
                &format!(
                    "if self.input[self.position + 1] == '{}' {{",
                    ch.as_str().unwrap()
                ),
            );
            module.push_tabln(
                5,
                &format!(
                    "let entity = vec!['{}','{}'];",
                    prefix.as_str().unwrap(),
                    ch.as_str().unwrap()
                ),
            );
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(5, "return token::Token::ENTITYTAG(entity);");
            module.push_tabln(4, "} else {");
            module.push_tabln(5, "tok = token::Token::CH(self.ch);");
            module.push_tabln(4, "}");
            module.push_tabln(3, "}");
        }
    }

    if let Some(val) = h.get(&Yaml::String("single_keyword".to_string())) {
        if let Some(list) = val.as_vec() {
            for v in list {
                module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
                module.push_tabln(4, "tok = token::Token::KEYWORD(vec![self.ch]);");
                module.push_tabln(3, "}");
            }
        }
    }

    if let Some(val) = h.get(&Yaml::String("single_constant".to_string())) {
        if let Some(list) = val.as_vec() {
            for v in list {
                module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
                module.push_tabln(4, "tok = token::Token::STRING(vec![self.ch]);");
                module.push_tabln(3, "}");
            }
        }
    }

    if let Some(v) = get_condition(h).get(&Yaml::String(ACCEPT_PREFIX_KEYWORD.to_string())) {
        module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
        module.push_tabln(4, "if is_letter(self.input[self.position + 1]) {");
        module.push_tabln(5, "self.read_char();");
        module.push_tabln(
            5,
            &format!("let mut identifier = vec!['{}'];", v.as_str().unwrap()),
        );
        module.push_tabln(5, "identifier.append(&mut read_identifier(self));");
        module.push_tabln(5, "return token::Token::KEYWORD(identifier);");
        module.push_tabln(4, "}");
        module.push_tabln(4, "tok = token::Token::CH(self.ch);");
        module.push_tabln(3, "}");
    }

    if let Some(v) = get_condition(h).get(&Yaml::String(ACCEPT_PREFIX_VAR.to_string())) {
        module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
        module.push_tabln(4, "if is_letter(self.input[self.position + 1]) {");
        if let Some(pref) = get_condition(h).get(&Yaml::String(VAR_CONSTANT_PREFIX.to_string())) {
            module.push_tabln(5, "let position = self.position;");
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(
                5,
                &format!("let mut identifier = vec!['{}'];", v.as_str().unwrap()),
            );
            module.push_tabln(5, "identifier.append(&mut read_identifier(self));");
            module.push_tabln(
                5,
                &format!(
                    "if self.input[position-1] == '{}' {{",
                    pref.as_str().unwrap()
                ),
            );
            module.push_tabln(6, "return token::Token::CONSTANT(identifier);");
            module.push_tabln(5, "} else {");
            module.push_tabln(6, "return token::Token::VAR(identifier);");
            module.push_tabln(5, "}");
        } else {
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(
                5,
                &format!("let mut identifier = vec!['{}'];", v.as_str().unwrap()),
            );
            module.push_tabln(5, "identifier.append(&mut read_identifier(self));");
            module.push_tabln(5, "return token::Token::VAR(identifier);");
        }
        module.push_tabln(4, "}");
        module.push_tabln(4, "tok = token::Token::CH(self.ch);");
        module.push_tabln(3, "}");
    }

    if slash_comment_enable(h) {
        module.push_tabln(3, "'/' => {");
        module.push_tabln(4, "if self.input[self.position + 1] == '/' {");
        module.push_tabln(5, "tok = token::Token::COMMENT(read_slash_comment(self));");
        if slash_star_comment_enable(h) {
            module.push_tabln(4, "} else if self.input[self.position + 1] == '*' {");
            module.push_tabln(
                5,
                "tok = token::Token::COMMENT(read_slash_star_comment(self));",
            );
        }
        module.push_tabln(4, "} else {");
        module.push_tabln(5, "tok = token::Token::CH(self.ch);");
        module.push_tabln(4, "}");
        module.push_tabln(3, "}");
    } else if slash_star_comment_enable(h) {
        module.push_tabln(3, "'/' => {");
        module.push_tabln(4, "if self.input[self.position + 1] == '*' {");
        module.push_tabln(
            5,
            "tok = token::Token::COMMENT(read_slash_star_comment(self));",
        );
        module.push_tabln(4, "}");
        module.push_tabln(4, "self.read_char();");
        module.push_tabln(4, "return tok;");
        module.push_tabln(3, "}");
    }

    module.push_tabln(3, "_ => {");
    module.push_tabln(4, "return if is_letter(self.ch) {");
    module.push_tabln(5, "#[allow(unused_variables)]");
    module.push_tabln(5, "let start_position = self.position;");
    module.push_tabln(5, "#[allow(unused_mut)]");
    module.push_tabln(5, "let mut identifier: Vec<char> = read_identifier(self);");

    if let Some(val_prefix) =
        get_condition(h).get(&Yaml::String(ACCEPT_ENTITY_TAG_SUFFIX.to_string()))
    {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String(BREAK_ENTITY_TAG_SUFFIX.to_string()))
        {
            let val_break = val.as_str().unwrap();
            module.push_tab(5, &format!("if {} ", val_condition));
            module.push_strln("{");
            module.push_tabln(6, "let position = self.position;");
            module.push_tabln(6, "while self.position < self.input.len() {");
            module.push_tab(7, &format!("if {} ", val_break));
            module.push_strln("{");
            module.push_tabln(8, "break;");
            module.push_tabln(7, "}");
            module.push_tabln(7, "self.read_char();");
            module.push_tabln(6, "}");
            module.push_tabln(
                6,
                "identifier.append(&mut self.input[position..self.position].to_vec());",
            );
            module.push_tabln(5, "}")
        }
    }

    module.push_tabln(5, "match token::get_keyword_token(&identifier) {");
    module.push_tabln(7, "Ok(keyword_token) => {");
    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_CONSTANT_SUFFIX_KEYWORD.to_string()))
    {
        if let Some(suffix) = get_condition(h).get(&Yaml::String(CONSTANT_SUFFIX_CHAR.to_string()))
        {
            if let Some(value) =
                get_condition(h).get(&Yaml::String(CONSTANT_SUFFIX_KEYWORD.to_string()))
            {
                module.push_tabln(
                    8,
                    &format!(
                        "if self.ch == '{}' && identifier.iter().collect::<String>() == \"{}\" {{",
                        suffix.as_str().unwrap(),
                        value.as_str().unwrap(),
                    ),
                );
                module.push_tabln(9, "return token::Token::CONSTANT(identifier);");
                module.push_tabln(8, "}");
            }
        }
    }
    module.push_tabln(8, "keyword_token");
    module.push_tabln(7, "},");
    module.push_tabln(7, "Err(_err) => {");

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String(ACCEPT_ENTITY_PREFIX.to_string()))
    {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String(BREAK_ENTITY_PREFIX.to_string())) {
            let val_break = val.as_str().unwrap();
            module.push_tab(8, &format!("if {} ", val_condition));
            module.push_strln("{");
            module.push_tabln(9, "let position = self.position;");
            module.push_tabln(9, "while self.position < self.input.len() {");
            module.push_tab(10, &format!("if {} ", val_break));
            module.push_strln("{");
            module.push_tabln(11, "break;");
            module.push_tabln(10, "}");
            module.push_tabln(10, "self.read_char();");
            module.push_tabln(9, "}");
            module.push_tabln(
                9,
                "identifier.append(&mut self.input[position..self.position].to_vec());",
            );
            module.push_tabln(9, "return token::Token::ENTITY(identifier)");
            module.push_tabln(8, "}")
        }
    }

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String(ACCEPT_IDENT_SUFFIX.to_string())) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String(BREAK_IDENT_SUFFIX.to_string())) {
            let val_break = val.as_str().unwrap();
            module.push_tab(8, &format!("if {} ", val_condition));
            module.push_strln("{");
            module.push_tabln(9, "let position = self.position;");
            module.push_tabln(9, "while self.position < self.input.len() {");
            module.push_tab(10, &format!("if {} ", val_break));
            module.push_strln("{");
            module.push_tabln(11, "break;");
            module.push_tabln(10, "}");
            module.push_tabln(10, "self.read_char();");
            module.push_tabln(9, "}");
            module.push_tabln(
                9,
                "identifier.append(&mut self.input[position..self.position].to_vec());",
            );
            module.push_tabln(9, "return token::Token::IDENT(identifier)");
            module.push_tabln(8, "}")
        }
    }

    if let Some(val_prefix) =
        get_condition(h).get(&Yaml::String("ACCEPT_ENTITY_SUFFIX".to_string()))
    {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String("BREAK_ENTITY_SUFFIX".to_string())) {
            let val_break = val.as_str().unwrap();
            module.push_tab(8, &format!("if {} ", val_condition));
            module.push_strln("{");
            module.push_tabln(9, "let position = self.position;");
            module.push_tabln(9, "while self.position < self.input.len() {");
            module.push_tab(10, &format!("if {} ", val_break));
            module.push_strln("{");
            module.push_tabln(11, "break;");
            module.push_tabln(10, "}");
            module.push_tabln(10, "self.read_char();");
            module.push_tabln(9, "}");
            module.push_tabln(
                9,
                "identifier.append(&mut self.input[position..self.position].to_vec());",
            );
            module.push_tabln(9, "return token::Token::ENTITY(identifier)");
            module.push_tabln(8, "}")
        }
    }

    if let Some(_) =
        get_condition(h).get(&Yaml::String(ACCEPT_CONSTANT_SUFFIX_IDENTIFIER.to_string()))
    {
        if let Some(ch) = get_condition(h).get(&Yaml::String(ACCEPT_DASH_IDENTIFIER.to_string())) {
            module.push_tabln(8, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(9, "let last_position = self.position;");
            module.push_tabln(9, "self.read_char();");
            module.push_tabln(
                9,
                "while self.position < self.input.len() && is_letter(self.ch) {",
            );
            module.push_tabln(10, "self.read_char();");
            module.push_tabln(9, "}");
            module.push_tabln(
                9,
                "identifier.append(&mut self.input[last_position..self.position].to_vec());",
            );
            module.push_tabln(8, "}");
        }

        if let Some(ch) = get_condition(h).get(&Yaml::String(CONSTANT_SUFFIX_CHAR.to_string())) {
            module.push_tabln(8, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(9, "return token::Token::CONSTANT(identifier);");
            module.push_tabln(8, "}");
        }
    }

    for (_k, v) in get_entity_prefix(h) {
        module.push_tabln(
            8,
            &format!(
                "if start_position != 0 && self.input[start_position-1] == '{}' {{",
                v.as_str().unwrap()
            ),
        );
        module.push_tabln(9, "return token::Token::ENTITY(identifier)");
        module.push_tabln(8, "}");
    }

    for (_k, v) in get_entity_suffix(h) {
        module.push_tabln(8, &format!("if self.ch == '{}' {{", v.as_str().unwrap()));
        module.push_tabln(9, "return token::Token::ENTITY(identifier)");
        module.push_tabln(8, "}");
    }

    module.push_tabln(8, "token::Token::IDENT(identifier)");
    module.push_tabln(7, "}");
    module.push_tabln(6, "}");
    module.push_tabln(5, "} else if is_digit(self.ch) {");
    if let Some(ch) = get_condition(h).get(&Yaml::String(ACCEPT_SUFFIX_DIGIT.to_string())) {
        module.push_tabln(6, "let mut identifier: Vec<char> = read_number(self);");
        module.push_tabln(6, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
        module.push_tabln(7, "identifier.append(&mut vec![self.ch]);");
        module.push_tabln(7, "self.read_char();");
        module.push_tabln(6, "}");
    } else {
        module.push_tabln(6, "let identifier: Vec<char> = read_number(self);");
    }
    module.push_tabln(6, "token::Token::INT(identifier)");

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_STRING_ONE_QUOTE.to_string())) {
        module.push_tabln(5, "} else if self.ch == '\\'' {");
        module.push_tabln(6, "let str_value: Vec<char> = read_string(self, '\\'');");
        module.push_tabln(6, "token::Token::STRING(str_value)");
    }

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_STRING_DOUBLE_QUOTE.to_string())) {
        module.push_tabln(5, "} else if self.ch == '\"' {");
        module.push_tabln(6, "let str_value: Vec<char> = read_string(self, '\"');");
        module.push_tabln(6, "token::Token::STRING(str_value)");
    }

    module.push_tabln(5, "} else {");
    module.push_tabln(6, "token::Token::ILLEGAL");
    module.push_tabln(5, "}");
    module.push_tabln(4, "}");

    module.push_tabln(3, "}");
    module.push_tabln(2, "self.read_char();");
    module.push_tabln(2, "tok");
    module.push_tabln(1, "}");
    module.push_strln("}");
}

fn write_handle_xml_comment() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "if self.ch == '<' {");
    module.push_tabln(3, "let next_ch = self.input[self.position + 1];");
    module.push_tabln(3, "if self.position + 3 < self.input.len() && next_ch == '!' && self.input[self.position+2] == '-' && self.input[self.position+3] == '-' {");
    module.push_tabln(4, "let mut comment = vec!['&','l','t',';','!','-','-'];");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "let last_position = self.position;");
    module.push_tabln(4, "while self.position < self.input.len() {");
    module.push_tabln(5, "if self.ch == '-' {");
    module.push_tabln(6, "if self.input[self.position+1] == '-' {");
    module.push_tabln(7, "if self.input[self.position+2] == '>' {");
    module.push_tabln(7, "self.read_char();");
    module.push_tabln(7, "self.read_char();");
    module.push_tabln(7, "self.read_char();");
    module.push_tabln(7, "break;");
    module.push_tabln(7, "}");
    module.push_tabln(6, "}");
    module.push_tabln(5, "}");
    module.push_tabln(5, "self.read_char();");
    module.push_tabln(4, "}");
    module.push_tabln(
        4,
        "comment.append(&mut self.input[last_position..self.position].to_vec());",
    );
    module.push_tabln(4, "return token::Token::COMMENT(comment);");
    module.push_tabln(3, "}");
    module.push_tabln(2, "}");
    module.to_string()
}

pub fn generate_module(h: &Hash) -> String {
    let mut module = StringBuilder::new();
    module.push_strln("// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //");
    module.push_strln("pub mod token;");
    module.push_strln("pub mod render;\n");

    write_struct_lexer(&mut module);
    write_helper_is_letter(&mut module);
    write_helper_is_digit(&mut module);
    write_impl_lexer(&mut module, h);

    module.to_string()
}
