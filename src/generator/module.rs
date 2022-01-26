use crate::generator::{
    bracket_dash_comment_enable, double_dash_comment_enable, get_double_keyword, get_entity_prefix,
    get_entity_suffix, get_xml_entity_tag, hashtag_comment_enable, slash_comment_enable,
    slash_star_comment_enable, string::StringBuilder, xml_comment_enable, ConditionExt,
};
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const ACCEPT_PREFIX_KEYWORD_NEXT: &str = "ACCEPT_PREFIX_KEYWORD_NEXT";
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
const SKIP_NON_CHAR_LETTER_PREFIX: &str = "SKIP_NON_CHAR_LETTER_PREFIX";
const ACCEPT_STRING_EOF: &str = "ACCEPT_STRING_EOF";
const MARK_ENTITY_TAG_SUFFIX: &str = "MARK_ENTITY_TAG_SUFFIX";
const MARK_STRING_ENTITY_TAG: &str = "MARK_STRING_ENTITY_TAG";
const ACCEPT_DOUBLE_BRACKET_STRING: &str = "ACCEPT_DOUBLE_BRACKET_STRING";
const ACCEPT_ESCAPED_STRING: &str = "ACCEPT_ESCAPED_STRING";
const ACCEPT_HEXADECIMAL_NUMBER: &str = "ACCEPT_HEXADECIMAL_NUMBER";
const ACCEPT_ENTITY_SUFFIX: &str = "ACCEPT_ENTITY_SUFFIX";
const BREAK_ENTITY_SUFFIX: &str = "BREAK_ENTITY_SUFFIX";
const ACCEPT_CHAR_IDENTIFIER: &str = "ACCEPT_CHAR_IDENTIFIER";
const PREFIX_ONE_LINE_COMMENT: &str = "PREFIX_ONE_LINE_COMMENT";
const MARK_KEYWORD_AS_ENTITY_ON_PREFIX: &str = "MARK_KEYWORD_AS_ENTITY_ON_PREFIX";
const MARK_AS_KEYWORD_ON_CHAR: &str = "MARK_AS_KEYWORD_ON_CHAR";
const MARKUP_HEAD: &str = "MARKUP_HEAD";
const IGNORE_INTEGER: &str = "IGNORE_INTEGER";

pub fn generate_module(h: &Hash) -> String {
    let initial_module = include_str!("stub/initial_module.stub");
    let mut module = StringBuilder::new();
    module.push_str(initial_module);

    if h.get_some_condition(MARK_STRING_ENTITY_TAG).is_some()
        || h.get_some_condition(MARK_ENTITY_TAG_SUFFIX).is_some()
        || get_entity_suffix(h).len() > 0
    {
        write_helper_is_white_space(&mut module);
    }
    write_impl_lexer(&mut module, h);

    module.to_string()
}

fn write_helper_is_white_space(module: &mut StringBuilder) {
    module.push_strln("fn is_white_space(ch: char) -> bool {");
    module.push_tabln(1, r#"ch == ' ' || ch == '\t' || ch == '\t' || ch == '\n'"#);
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
    if let Some(val) = h.get_some_condition(ACCEPT_CHAR_IDENTIFIER) {
        for ch in val.as_str().unwrap().chars() {
            module.push_tabln(4, &format!("if l.ch == '{}' {{", ch));
            module.push_tabln(5, "l.read_char();");
            module.push_tabln(4, "}");
        }
    }
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");

    if h.get_some_condition(ACCEPT_STRING_ONE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_DOUBLE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_EOF).is_some()
    {
        module.push_str(&write_handle_read_string(h.clone()));
    }

    module.push_tabln(2, "let read_number = |l: &mut Lexer| -> Vec<char> {");
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "while l.position < l.input.len() && is_digit(l.ch) {");
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");

    if slash_comment_enable(h) {
        module.push_str(&write_handle_read_slash_comment());
    }

    if slash_star_comment_enable(h) {
        module.push_str(&write_handle_read_slash_star_comment())
    }

    if let Some(ch) = h.get_some_condition(MARK_AS_KEYWORD_ON_CHAR) {
        let c = ch.as_str().unwrap();
        module.push_tabln(2, &format!("if self.ch == '{}' {{", c));
        module.push_tabln(
            3,
            &format!("return token::Token::KEYWORD(read_string(self, '{}'));", c),
        );
        module.push_tabln(2, "}");
    }

    module.push_tabln(2, "let tok: token::Token;");

    if xml_comment_enable(h) {
        module.push_str(&write_handle_xml_comment());
    }

    if h.get_some_condition(ACCEPT_DOUBLE_BRACKET_STRING).is_some() {
        module.push_str(include_str!("stub/handle_double_bracket_string.stub"));
    }

    if double_dash_comment_enable(h) {
        module.push_str(&write_handle_double_dash_comment());
    }

    if bracket_dash_comment_enable(h) {
        module.push_str(&write_handle_bracket_dash_comment());
    }

    if hashtag_comment_enable(h) {
        module.push_str(include_str!("stub/handle_hashtag_comment.stub"));
    }

    if let Some(ch) = h.get_some_condition(SKIP_NON_CHAR_LETTER_PREFIX) {
        module.push_str(&write_handle_skip_non_char_letter(ch));
    }

    if h.get_some_condition(ACCEPT_STRING_EOF).is_some() {
        module.push_str(&write_handle_eof_string());
    }

    if let Some(ch) = h.get_some_condition(MARKUP_HEAD) {
        module.push_str(&write_handle_markup_head(ch.as_str().unwrap()));
    }

    for (_, val) in get_double_keyword(h) {
        let v: Vec<char> = val.as_str().unwrap().chars().collect();
        let first = v[0];
        let last = v[1];
        module.push_tab(2, "if self.read_position < self.input.len() ");
        module.push_str(&format!("&& self.ch == '{}' ", first));
        module.push_strln(&format!(
            "&& self.input[self.read_position] == '{}' {{",
            last
        ));
        module.push_tabln(3, "self.read_char();");
        module.push_tabln(3, "self.read_char();");
        module.push_tabln(
            3,
            &format!(
                "return token::Token::KEYWORD(vec!['{}', '{}']);",
                first, last
            ),
        );
        module.push_tabln(2, "}\n");
    }

    if let Some(ch) = h.get_some_condition(PREFIX_ONE_LINE_COMMENT) {
        let v = ch.as_str().unwrap();
        let chars: Vec<char> = v.chars().collect();
        let first = chars[0];
        let last = chars[1];
        module.push_tab(2, "if self.read_position < self.input.len() ");
        module.push_str(&format!("&& self.ch == '{}' ", first));
        module.push_strln(&format!(
            "&& self.input[self.read_position] == '{}' {{",
            last
        ));
        module.push_tabln(3, "return token::Token::COMMENT(read_string(self, '\\n'));");
        module.push_tabln(2, "}\n");
    }

    for (_, val) in get_double_keyword(h) {
        let v: Vec<char> = val.as_str().unwrap().chars().collect();
        let first = v[0];
        let last = v[1];
        module.push_tab(2, "if self.read_position < self.input.len() ");
        module.push_str(&format!("&& self.ch == '{}' ", first));
        module.push_strln(&format!(
            "&& self.input[self.read_position] == '{}' {{",
            last
        ));
        module.push_tabln(3, "self.read_char();");
        module.push_tabln(3, "self.read_char();");
        module.push_tabln(
            3,
            &format!(
                "return token::Token::KEYWORD(vec!['{}', '{}']);",
                first, last
            ),
        );
        module.push_tabln(2, "}\n");
    }

    module.push_tabln(2, "match self.ch {");
    module.push_tabln(3, "'\\n' => {");
    module.push_tabln(4, "tok = token::Token::ENDL(self.ch);");
    module.push_tabln(3, "}");

    module.push_tabln(3, "'\\0' => {");
    module.push_tabln(4, "tok = token::Token::EOF;");
    module.push_tabln(3, "}");

    if h.get_some_condition(ACCEPT_HEXADECIMAL_NUMBER).is_some() {
        module.push_str(&write_handle_hexadecimal());
    }

    if let Some(prefix) = h.get_some_condition(ACCEPT_ENTITY_TAG_PREFIX) {
        if let Some(ch) = h.get_some_condition(ENTITY_TAG_PREFIX_CHAR) {
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

    if let Some(prefix) = h.get_some_condition(ENTITY_TAG_PREFIX_CHAR) {
        if let Some(ch) = h.get_some_condition(ENTITY_CLOSE_TAG_SUFFIX_CHAR) {
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

    if let Some(v) = h.get_some_condition(ACCEPT_PREFIX_KEYWORD) {
        module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
        if let Some(ch) = h.get_some_condition(ACCEPT_PREFIX_KEYWORD_NEXT) {
            module.push_tabln(4, "let next_ch = self.input[self.position + 1];");
            module.push_tab(4, "if is_letter(next_ch) ");
            module.push_strln(&format!("|| next_ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(5, "let mut identifier = vec![self.ch];");
            module.push_tabln(5, "self.read_char();");
            module.push_tabln(
                5,
                &format!(
                    "if self.input[self.position + 1] == '{}' {{",
                    ch.as_str().unwrap()
                ),
            );
            module.push_tabln(6, "self.read_char();");
            module.push_tabln(6, "identifier.append(&mut vec![self.ch]);");
            module.push_tabln(5, "}");
        } else {
            module.push_tabln(4, "if is_letter(self.input[self.position + 1]) {");
            module.push_tabln(5, "let mut identifier = vec![self.ch];");
            module.push_tabln(5, "self.read_char();");
        }

        module.push_tabln(5, "identifier.append(&mut read_identifier(self));");
        module.push_tabln(5, "return token::Token::KEYWORD(identifier);");
        module.push_tabln(4, "}");
        module.push_tabln(4, "tok = token::Token::CH(self.ch);");
        module.push_tabln(3, "}");
    }

    if let Some(v) = h.get_some_condition(ACCEPT_PREFIX_VAR) {
        module.push_tabln(3, &format!("'{}' => {{", v.as_str().unwrap()));
        module.push_tabln(4, "if is_letter(self.input[self.position + 1]) {");
        if let Some(pref) = h.get_some_condition(VAR_CONSTANT_PREFIX) {
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

    if let Some(val_prefix) = h.get_some_condition(ACCEPT_ENTITY_TAG_SUFFIX) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = h.get_some_condition(BREAK_ENTITY_TAG_SUFFIX) {
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
    if h.get_some_condition(ACCEPT_CONSTANT_SUFFIX_KEYWORD)
        .is_some()
    {
        if let Some(suffix) = h.get_some_condition(CONSTANT_SUFFIX_CHAR) {
            if let Some(value) = h.get_some_condition(CONSTANT_SUFFIX_KEYWORD) {
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

    if let Some(ch) = h.get_some_condition(MARK_KEYWORD_AS_ENTITY_ON_PREFIX) {
        module.push_tabln(
            8,
            "if start_position >= 1 && self.input[start_position - 1] ==",
        );
        module.push_strln(&format!("'{}' {{", ch.as_str().unwrap()));
        module.push_tabln(9, "return token::Token::ENTITY(identifier)");
        module.push_tabln(8, "}");
    }

    if get_xml_entity_tag(h).len() >= 1 {
        module.push_tabln(8, "if start_position - 1 != 0");
        module.push_tabln(9, "&& self.input[start_position-1] == '<'");
        module.push_tabln(9, "|| self.input[start_position-1] == '/'");
        module.push_tabln(9, "|| self.ch == '>' {");
        module.push_tabln(9, "return keyword_token");
        module.push_tabln(8, "}");
        module.push_tabln(8, "return token::Token::IDENT(identifier);");
    } else {
        module.push_tabln(8, "keyword_token");
    }
    module.push_tabln(7, "},");
    module.push_tabln(7, "Err(_err) => {");

    if let Some(val_prefix) = h.get_some_condition(ACCEPT_ENTITY_PREFIX) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = h.get_some_condition(BREAK_ENTITY_PREFIX) {
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

    if let Some(val_prefix) = h.get_some_condition(ACCEPT_IDENT_SUFFIX) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = h.get_some_condition(BREAK_IDENT_SUFFIX) {
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

    if let Some(val_prefix) = h.get_some_condition(ACCEPT_ENTITY_SUFFIX) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = h.get_some_condition(BREAK_ENTITY_SUFFIX) {
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

    if h.get_some_condition(ACCEPT_CONSTANT_SUFFIX_IDENTIFIER)
        .is_some()
    {
        if let Some(ch) = h.get_some_condition(ACCEPT_DASH_IDENTIFIER) {
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

        if let Some(ch) = h.get_some_condition(CONSTANT_SUFFIX_CHAR) {
            module.push_tabln(8, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(9, "return token::Token::CONSTANT(identifier);");
            module.push_tabln(8, "}");
        }
    }

    for (_k, v) in get_entity_prefix(h) {
        let ch = v.as_str().unwrap();
        module.push_tab(8, "if start_position > 0 ");
        module.push_strln(&format!("&& self.input[start_position - 1] == '{}' {{", ch));
        module.push_tabln(9, "return token::Token::ENTITY(identifier)");
        module.push_tabln(8, "}");
    }

    if let Some(ch) = h.get_some_condition(MARK_ENTITY_TAG_SUFFIX) {
        module.push_tabln(8, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
        module.push_tabln(9, "return token::Token::ENTITYTAG(identifier)");
        module.push_tabln(8, "}");
    }

    for (_k, ch) in get_entity_suffix(h) {
        module.push_tabln(8, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
        module.push_tabln(9, "return token::Token::ENTITY(identifier);");
        module.push_tabln(8, "} else if is_white_space(self.ch) {");
        module.push_tabln(9, "let start_position = self.position;");
        module.push_tabln(9, "let mut position = self.position;");
        module.push_tabln(9, "let mut ch = self.input[position];");
        module.push_tabln(
            9,
            "while position < self.input.len() && is_white_space(ch) {",
        );
        module.push_tabln(10, "position = position + 1;");
        module.push_tabln(10, "if position < self.input.len() {");
        module.push_tabln(11, "ch = self.input[position];");
        module.push_tabln(10, "}");
        module.push_tabln(9, "}");
        module.push_tabln(9, &format!("if ch == '{}' {{", ch.as_str().unwrap()));
        module.push_tabln(10, "self.position = position - 1;");
        module.push_tabln(10, "self.read_position = position;");
        module.push_tabln(10, "let mut value = identifier;");
        module.push_tabln(
            10,
            "value.append(&mut self.input[start_position..self.position].to_vec());",
        );
        module.push_tabln(10, "return token::Token::ENTITY(value)");
        module.push_tabln(9, "}");
        module.push_tabln(8, "}");
    }

    module.push_tabln(8, "token::Token::IDENT(identifier)");
    module.push_tabln(7, "}");
    module.push_tabln(6, "}");
    module.push_tabln(5, "} else if is_digit(self.ch) {");
    if let Some(ch) = h.get_some_condition(ACCEPT_SUFFIX_DIGIT) {
        module.push_tabln(6, "let mut identifier: Vec<char> = read_number(self);");
        module.push_tabln(6, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
        module.push_tabln(7, "identifier.append(&mut vec![self.ch]);");
        module.push_tabln(7, "self.read_char();");
        module.push_tabln(6, "}");
    } else {
        module.push_tabln(6, "let identifier: Vec<char> = read_number(self);");
    }

    if h.get_some_condition(IGNORE_INTEGER).is_some() {
        module.push_tabln(6, "token::Token::IDENT(identifier)");
    } else {
        module.push_tabln(6, "token::Token::INT(identifier)");
    }

    if h.get_some_condition(ACCEPT_STRING_ONE_QUOTE).is_some() {
        module.push_tabln(5, "} else if self.ch == '\\'' {");
        module.push_tabln(6, "let str_value: Vec<char> = read_string(self, '\\'');");
        module.push_tabln(6, "token::Token::STRING(str_value)");
    }

    if h.get_some_condition(ACCEPT_STRING_DOUBLE_QUOTE).is_some() {
        module.push_tabln(5, "} else if self.ch == '\"' {");
        module.push_tabln(6, "let str_value: Vec<char> = read_string(self, '\"');");
        if let Some(ch) = h.get_some_condition(MARK_STRING_ENTITY_TAG) {
            module.push_tabln(6, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(7, "return token::Token::ENTITYTAG(str_value);");
            module.push_tabln(6, "} else if is_white_space(self.ch) {");
            module.push_tabln(7, "let start_position = self.position;");
            module.push_tabln(7, "let mut position = self.position;");
            module.push_tabln(7, "let mut ch = self.input[position];");
            module.push_tabln(
                7,
                "while position < self.input.len() && is_white_space(ch) {",
            );
            module.push_tabln(8, "position = position + 1;");
            module.push_tabln(8, "ch = self.input[position];");
            module.push_tabln(7, "}");
            module.push_tabln(7, &format!("if ch == '{}' {{", ch.as_str().unwrap()));
            module.push_tabln(8, "self.position = position;");
            module.push_tabln(8, "self.read_position = position + 1;");
            module.push_tabln(8, "let mut value = str_value;");
            module.push_tabln(
                8,
                "value.append(&mut self.input[start_position..self.read_position].to_vec());",
            );
            module.push_tabln(8, "return token::Token::ENTITYTAG(value)");
            module.push_tabln(7, "}");
            module.push_tabln(6, "}");
        }
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

fn write_handle_markup_head(head: &str) -> String {
    let heads: Vec<char> = head.to_string().chars().collect();
    let scope = include_str!("stub/markup_head_scope.stub").to_string();
    let template = include_str!("stub/markup_head.stub").to_string();
    let mut code = String::new();
    for index in 0..heads.len() {
        let next_position = index + 2;
        let h = heads[0..index + 1].to_vec();
        let vec_char = h.iter().map(|c| format!("'{}',", c)).collect::<String>();
        let result = template
            .replace("{index}", &next_position.to_string())
            .replace("{mark}", &vec_char);
        code.push_str(&result);
    }
    scope
        .replace("{mark}", &heads.first().unwrap().to_string())
        .replace("{scope}", &code)
}

fn write_handle_read_slash_star_comment() -> String {
    let mut module = StringBuilder::new();
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
    module.push_tabln(2, "};\n");
    module.to_string()
}

fn write_handle_read_slash_comment() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "let read_slash_comment = |l: &mut Lexer| -> Vec<char> {");
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "while l.position < l.input.len() {");
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(
        4,
        "if l.position + 1 < l.input.len() && l.input[l.position + 1] == '\\n' {",
    );
    module.push_tabln(5, "break;");
    module.push_tabln(4, "}");
    module.push_tabln(3, "}");
    module.push_tabln(3, "if l.position + 1 >= l.input.len(){");
    module.push_tabln(4, "return l.input[position..l.position].to_vec();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position + 1].to_vec()");
    module.push_tabln(2, "};\n");
    module.to_string()
}

fn write_handle_read_string(h: Hash) -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(
        2,
        "let read_string = |l: &mut Lexer, ch: char| -> Vec<char> {",
    );
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "l.read_char();");
    module.push_tabln(3, "while l.position < l.input.len() && l.ch != ch {");
    if h.get_some_condition(ACCEPT_ESCAPED_STRING).is_some() {
        module.push_tabln(4, "if l.ch == '\\\\' {");
        module.push_tabln(5, "l.read_char()");
        module.push_tabln(4, "}");
    }
    module.push_tabln(4, "l.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.read_char();");
    module.push_tabln(3, "if l.position > l.input.len() {");
    module.push_tabln(4, "l.position = l.position - 1;");
    module.push_tabln(4, "l.read_position = l.read_position - 1;");
    module.push_tabln(3, "}");
    module.push_tabln(3, "l.input[position..l.position].to_vec()");
    module.push_tabln(2, "};\n");
    module.to_string()
}

fn write_handle_hexadecimal() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(3, "'0' => {");
    module.push_tabln(4, "return if self.input[self.read_position] == 'x' {");
    module.push_tabln(5, "let start_position = self.position;");
    module.push_tabln(5, "self.read_char();");
    module.push_tabln(5, "self.read_char();");
    module.push_tabln(
        5,
        "while self.position < self.input.len() && (is_digit(self.ch) || is_letter(self.ch)) {",
    );
    module.push_tabln(6, "self.read_char()");
    module.push_tabln(5, "}");
    module.push_tabln(
        5,
        "let hexadecimal = &self.input[start_position..self.position];",
    );
    module.push_tabln(5, "token::Token::INT(hexadecimal.to_vec())");
    module.push_tabln(4, "} else {");
    module.push_tabln(5, "let number = read_number(self);");
    module.push_tabln(5, "token::Token::INT(number)");
    module.push_tabln(4, "}");
    module.push_tabln(3, "}");
    module.to_string()
}

fn write_handle_skip_non_char_letter(ch: Yaml) -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, &format!("if self.ch == '{}' {{", ch.as_str().unwrap()));
    module.push_tabln(3, "let start_position = self.position;");
    module.push_tabln(3, "while self.position < self.input.len()");
    module.push_tabln(4, "&& !is_letter(self.ch)");
    module.push_tabln(4, "&& !is_digit(self.ch)");
    module.push_tabln(4, "&& self.ch != '\\n'");
    module.push_tabln(4, "&& self.ch != ' '");
    module.push_tabln(3, "{");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(3, "}");
    module.push_tabln(
        3,
        "let identifier = self.input[start_position..self.position].to_vec();",
    );
    module.push_tabln(3, "return token::Token::IDENT(identifier)");
    module.push_tabln(2, "}\n");
    module.to_string()
}

fn write_handle_xml_comment() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "if self.ch == '<' {");
    module.push_tabln(3, "let next_ch = self.input[self.position + 1];");
    module.push_tabln(3, "if self.position + 3 < self.input.len()");
    module.push_tabln(4, "&& next_ch == '!'");
    module.push_tabln(4, "&& self.input[self.position + 2] == '-'");
    module.push_tabln(4, "&& self.input[self.position + 3] == '-'");
    module.push_tabln(3, "{");
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
    module.push_tabln(8, "self.read_char();");
    module.push_tabln(8, "self.read_char();");
    module.push_tabln(8, "self.read_char();");
    module.push_tabln(8, "break;");
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
    module.push_tabln(2, "}\n");
    module.to_string()
}

fn write_handle_double_dash_comment() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "if self.ch == '-' {");
    module.push_tabln(3, "let next_ch = self.input[self.position + 1];");
    module.push_tabln(
        3,
        "if self.position + 1 < self.input.len() && next_ch == '-' {",
    );
    module.push_tabln(4, "let mut comment = vec!['-','-'];");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "let last_position = self.position;");
    module.push_tabln(4, "while self.position < self.input.len() {");
    module.push_tabln(5, "if self.ch == '\\n' {");
    module.push_tabln(6, "break;");
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

fn write_handle_bracket_dash_comment() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "if self.ch == '{' {");
    module.push_tabln(3, "let next_ch = self.input[self.position + 1];");
    module.push_tabln(
        3,
        "if self.position + 1 < self.input.len() && next_ch == '-' {",
    );
    module.push_tabln(4, "let mut comment = vec!['{','-'];");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "let last_position = self.position;");
    module.push_tabln(4, "while self.position < self.input.len() {");
    module.push_tabln(5, "if self.read_position <= self.input.len() && self.ch == '-' && self.input[self.read_position] == '}' {");
    module.push_tabln(6, "break;");
    module.push_tabln(5, "}");
    module.push_tabln(5, "self.read_char();");
    module.push_tabln(4, "}");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(
        4,
        "comment.append(&mut self.input[last_position..self.position].to_vec());",
    );
    module.push_tabln(4, "return token::Token::COMMENT(comment);");
    module.push_tabln(3, "}");
    module.push_tabln(2, "}");
    module.to_string()
}

fn write_handle_eof_string() -> String {
    let mut module = StringBuilder::new();
    module.push_tabln(2, "if self.ch == '<' {");
    module.push_tabln(3, "let next_ch = self.input[self.position + 1];");
    module.push_tab(3, "if self.position + 5 < self.input.len() ");
    module.push_str("&& next_ch == '<' ");
    module.push_str("&& self.input[self.position + 2] == 'E' ");
    module.push_str("&& self.input[self.position + 3] == 'O' ");
    module.push_str("&& self.input[self.position + 4] == 'F' ");
    module.push_strln("{");
    module.push_tabln(
        4,
        r#"let mut comment = String::from("<<EOF").chars().collect::<Vec<_>>();"#,
    );
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "self.read_char();");
    module.push_tabln(4, "let last_position = self.position;");
    module.push_tabln(4, "while self.position < self.input.len() {");
    module.push_tabln(5, "if self.ch == 'E' {");
    module.push_tabln(6, "if self.input[self.position+1] == 'O' {");
    module.push_tabln(7, "if self.input[self.position+2] == 'F' {");
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
    module.push_tabln(4, "return token::Token::STRING(comment);");
    module.push_tabln(3, "}");
    module.push_tabln(2, "}");
    module.to_string()
}
