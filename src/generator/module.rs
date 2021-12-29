use crate::generator::{
    get_condition, get_entity_prefix, get_entity_suffix, slash_comment_enable,
    slash_star_comment_enable, string::StringBuilder,
};
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const PREFIX_KEYWORD_CHAR: &str = "PREFIX_KEYWORD_CHAR";
const ACCEPT_ENTITY_TAG_SUFFIX: &str = "ACCEPT_ENTITY_TAG_SUFFIX";
const BREAK_ENTITY_TAG_SUFFIX: &str = "BREAK_ENTITY_TAG_SUFFIX";
const ACCEPT_ENTITY_PREFIX: &str = "ACCEPT_ENTITY_PREFIX";
const BREAK_ENTITY_PREFIX: &str = "BREAK_ENTITY_PREFIX";
const ACCEPT_IDENT_SUFFIX: &str = "ACCEPT_IDENT_SUFFIX";
const BREAK_IDENT_SUFFIX: &str = "BREAK_IDENT_SUFFIX";
const ACCEPT_STRING_ONE_QUOTE: &str = "ACCEPT_STRING_ONE_QUOTE";
const ACCEPT_STRING_DOUBLE_QUOTE: &str = "ACCEPT_STRING_DOUBLE_QUOTE";

fn write_struct_lexer(module: &mut StringBuilder) {
    module.push_strln("pub struct Lexer {");
    module.push_tabln(1, "input: Vec<char>,");
    module.push_tabln(1, "pub position: usize,");
    module.push_tabln(1, "pub read_position: usize,");
    module.push_tabln(1, "pub ch: char");
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
    module.push_tabln(3, "ch: '0'");
    module.push_tabln(2, "}");
    module.push_tabln(1, "}\n");

    module.push_tabln(1, "pub fn read_char(&mut self) {");
    module.push_tabln(2, "if self.read_position >= self.input.len() {");
    module.push_tabln(3, "self.ch = '0';");
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
        module.push_tabln(4, "if l.input[l.position+1] == '\\n' {");
        module.push_tabln(5, "break;");
        module.push_tabln(4, "}");
        module.push_tabln(3, "}");
        module.push_tabln(3, "l.input[position..l.position+1].to_vec()");
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
        module.push_tabln(4, "if l.input[l.position+1] == '*' {");
        module.push_tabln(5, "if l.input[l.position+2] == '/' {");
        module.push_tabln(6, "l.read_char();");
        module.push_tabln(6, "l.read_char();");
        module.push_tabln(6, "break;");
        module.push_tabln(5, "}");
        module.push_tabln(4, "}");
        module.push_tabln(4, "l.read_char();");
        module.push_tabln(3, "}");
        module.push_tabln(3, "l.input[position..l.position+1].to_vec()");
        module.push_tabln(2, "};\n")
    }

    module.push_tabln(2, "let tok: token::Token;");
    module.push_tabln(2, "match self.ch {");
    module.push_tabln(3, "'\\n' => {");
    module.push_tabln(4, "tok = token::Token::ENDL(self.ch);");
    module.push_tabln(3, "}");

    module.push_tabln(3, "'0' => {");
    module.push_tabln(4, "if self.position < self.input.len() {");
    module.push_tabln(
        5,
        "tok = token::Token::INT(self.input[self.position..self.position+1].to_vec());",
    );
    module.push_tabln(4, "} else {");
    module.push_tabln(5, "tok = token::Token::EOF;");
    module.push_tabln(4, "}");
    module.push_tabln(3, "}");

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_PREFIX_KEYWORD.to_string())) {
        if let Some(v) = get_condition(h).get(&Yaml::String(PREFIX_KEYWORD_CHAR.to_string())) {
            module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
            module.push_tabln(4, "if is_letter(self.input[self.position+1]) {");
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(5, "let mut identifier = vec!['@'];");
            module.push_tabln(5, "identifier.append(&mut read_identifier(self));");
            module.push_tabln(5, "return token::Token::KEYWORD(identifier);");
            module.push_tabln(4, "}");
            module.push_tabln(4, "tok = token::Token::CH(self.ch);");
            module.push_tabln(3, "}");
        }
    }

    if slash_comment_enable(h) {
        module.push_tabln(3, "'/' => {");
        module.push_tabln(4, "if self.input[self.position+1] == '/' {");
        module.push_tabln(5, "tok = token::Token::COMMENT(read_slash_comment(self));");
        if slash_star_comment_enable(h) {
            module.push_tabln(4, "} else if self.input[self.position+1] == '*' {");
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
        module.push_tabln(4, "if self.input[self.position+1] == '*' {");
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
    module.push_tabln(5, "let prev_pos = self.position;");
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

    for (_k, v) in get_entity_prefix(h) {
        module.push_tabln(
            8,
            &format!(
                "if prev_pos != 0 && self.input[prev_pos-1] == '{}' {{",
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
    module.push_tabln(6, "let identifier: Vec<char> = read_number(self);");
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

pub fn generate_module(h: &Hash) -> String {
    let mut module = StringBuilder::new();
    module.push_strln("// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //");
    module.push_strln("pub mod token;");
    module.push_strln("pub mod render;\n");

    write_struct_lexer(&mut module);
    write_helper_is_letter(&mut module);
    write_helper_is_digit(&mut module);
    write_impl_lexer(&mut module, h);

    module.to_string()
}
