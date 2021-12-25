use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;
use crate::generator::{
    get_base,
    get_condition,
    get_prefix,
    slash_comment_enable,
    slash_star_comment_enable,
    string::StringBuilder
};

pub fn generate_module(h: &Hash) -> String {
    let mut module = StringBuilder::new();
    module.push_strln("// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ----");
    module.push_strln("pub mod token;");
    module.push_strln("pub mod render;\n");

    module.push_strln("pub struct Lexer {");
    module.push_tabln(1, "input: Vec<char>,");
    module.push_tabln(1, "pub position: usize,");
    module.push_tabln(1, "pub read_position: usize,");
    module.push_tabln(1, "pub ch: char");
    module.push_strln("}\n");

    module.push_strln("fn is_letter(ch: char) -> bool {");
    module.push_tabln(1,"'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'");
    module.push_strln("}\n");

    module.push_strln("fn is_digit(ch: char) -> bool {");
    module.push_tabln(1, "'0' <= ch && ch <= '9'");
    module.push_strln("}\n");

    module.push_strln("impl Lexer {");
    module.push_tabln(1, "pub fn new(input: Vec<char>) -> Self {");
    module.push_tabln(2, "Self {");
    module.push_tabln(3, "input,");
    module.push_tabln(3, "position: 0,");
    module.push_tabln(3, "read_position: 0,");
    module.push_tabln(3, "ch: '0'");
    module.push_tabln(2, "}");
    module.push_tabln(1, "}");
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

    module.push_tabln(2, "let read_string = |l: &mut Lexer| -> Vec<char> {");
    module.push_tabln(3, "let position = l.position;");
    module.push_tabln(3, "l.read_char();");
    module.push_tabln(3, "while l.position < l.input.len() && l.ch != '\"' {");
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
        module.push_str("\t\tlet read_slash_comment = |l: &mut Lexer| -> Vec<char> {\n\
            \t\t\tlet position = l.position;\n\
            \t\t\twhile l.position < l.input.len() {\n\
                \t\t\t\tl.read_char();\n\
                \t\t\t\tif l.input[l.position+1] == '\\n' {\n\
                \t\t\t\t\tbreak;\n\
                \t\t\t\t}\n\
            \t\t\t}\n\
            \t\t\tl.input[position..l.position+1].to_vec()\n\
        \t\t};\n\n")
    }

    if slash_star_comment_enable(h) {
        module.push_str("\t\tlet read_slash_star_comment = |l: &mut Lexer| -> Vec<char> {\n\
            \t\t\tlet position = l.position;\n\
            \t\t\twhile l.position < l.input.len() {\n\
                \t\t\t\tif l.position == l.input.len() {\n\
                \t\t\t\t\tbreak;\n\
                \t\t\t\t}\n\
                \t\t\t\tif l.input[l.position+1] == '*' {\n\
                \t\t\t\t\tif l.input[l.position+2] == '/' {\n\
                \t\t\t\t\t\tl.read_char();\n\
                \t\t\t\t\t\tl.read_char();\n\
                \t\t\t\t\t\tbreak;\n\
                \t\t\t\t\t}\n\
                \t\t\t\t}\n\
                \t\t\t\tl.read_char();\n\
            \t\t\t}\n\
            \t\t\tl.input[position..l.position+1].to_vec()\n\
        \t\t};\n\n")
    }

    module.push_str("\t\tlet tok: token::Token;\n\
        \t\tmatch self.ch {\n\
    ");

    for (k, v) in get_base(h) {
        module.push_str(&format!("\t\t\t'{}' => ", v.as_str().unwrap()));
        module.push_str("{\n");
        module.push_str(&format!("\t\t\t\ttok = token::Token::{}(self.ch);\n", k.as_str().unwrap()));
        module.push_str("\t\t\t}\n");
    }

    module.push_str("\t\t\t'\\n' => {\n");
    module.push_str("\t\t\t\ttok = token::Token::ENDL(self.ch);\n");
    module.push_str("\t\t\t}\n");

    module.push_str("\t\t\t'0' => {\n");
    module.push_str("\t\t\t\tif self.position < self.input.len() {\n");
    module.push_str("\t\t\t\t\ttok = token::Token::INT(self.input[self.position..self.position+1].to_vec());\n");
    module.push_str("\t\t\t\t} else {\n");
    module.push_str("\t\t\t\t\ttok = token::Token::EOF;\n");
    module.push_str("\t\t\t\t}\n");
    module.push_str("\t\t\t}\n");

    if slash_comment_enable(h) {
        module.push_str("\t\t\t'/' => {\n");
        module.push_str("\t\t\t\tif self.input[self.position+1] == '/' {\n");
        module.push_str("\t\t\t\t\ttok = token::Token::COMMENT(read_slash_comment(self));\n");
        if slash_star_comment_enable(h) {
            module.push_str("\t\t\t\t} else if self.input[self.position+1] == '*' {\n");
            module.push_str("\t\t\t\t\ttok = token::Token::COMMENT(read_slash_star_comment(self));\n");
        }
        module.push_str("\t\t\t\t} else {\n");
        module.push_str("\t\t\t\t\ttok = token::Token::CH(self.ch);\n");
        module.push_str("\t\t\t\t}\n");
        module.push_str("\t\t\t}\n");
    } else if slash_star_comment_enable(h) {
        module.push_str("\t\t\t'/' => {\n");
        module.push_str("\t\t\t\tif self.input[self.position+1] == '*' {\n");
        module.push_str("\t\t\t\t\ttok = token::Token::COMMENT(read_slash_star_comment(self));\n");
        module.push_str("\t\t\t\t}\n");
        module.push_str("\t\t\t\tself.read_char();\n");
        module.push_str("\t\t\t\treturn tok;\n");
        module.push_str("\t\t\t}\n");
    }

    module.push_str("\t\t\t_ => {\n\
            \t\t\t\treturn if is_letter(self.ch) {\n\
                \t\t\t\t\tlet prev_pos = self.position;\n\
                \t\t\t\t\tlet mut identifier: Vec<char> = read_identifier(self);\n\
    ");

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String("ACCEPT_ENTITY_TAG_SUFFIX".to_string())) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String("BREAK_ENTITY_TAG_SUFFIX".to_string())) {
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
            module.push_tabln(6, "identifier.append(&mut self.input[position..self.position].to_vec());");
            module.push_tabln(5, "}")
        }
    }

    module.push_str("\t\t\t\t\tmatch token::get_keyword_token(&identifier) {\n\
                    \t\t\t\t\t\t\tOk(keyword_token) => {\n\
    ");

    for (k, v) in get_prefix(h) {
        if k.as_str().unwrap() == "ENTITY_TOKEN_SUFFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if self.ch == '{}' ", v.as_str().unwrap()));
            module.push_str("{\n\t\t\t\t\t\t\t\t\t");
            module.push_str("return token::Token::ENTITY(self.input[prev_pos..self.position].to_vec());\n");
            module.push_str("\t\t\t\t\t\t\t\t}\n");
            break;
        }
    }

    module.push_str("\t\t\t\t\t\t\t\tkeyword_token\n\
                    \t\t\t\t\t\t\t},\n\
                    \t\t\t\t\t\t\tErr(_err) => {\n\
    ");

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String("ACCEPT_ENTITY_PREFIX".to_string())) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String("BREAK_ENTITY_PREFIX".to_string())) {
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
            module.push_tabln(9, "identifier.append(&mut self.input[position..self.position].to_vec());");
            module.push_tabln(9, "return token::Token::ENTITY(identifier)");
            module.push_tabln(8, "}")
        }
    }

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String("ACCEPT_IDENT_SUFFIX".to_string())) {
        let val_condition = val_prefix.as_str().unwrap();
        if let Some(val) = get_condition(h).get(&Yaml::String("BREAK_IDENT_SUFFIX".to_string())) {
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
            module.push_tabln(9, "identifier.append(&mut self.input[position..self.position].to_vec());");
            module.push_tabln(9, "return token::Token::IDENT(identifier)");
            module.push_tabln(8, "}")
        }
    }

    if let Some(val_prefix) = get_condition(h).get(&Yaml::String("ACCEPT_ENTITY_SUFFIX".to_string())) {
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
            module.push_tabln(9, "identifier.append(&mut self.input[position..self.position].to_vec());");
            module.push_tabln(9, "return token::Token::ENTITY(identifier)");
            module.push_tabln(8, "}")
        }
    }

    for (k, v) in get_prefix(h) {
        if k.as_str().unwrap() == "ENTITY_PREFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if prev_pos != 0 && self.input[prev_pos-1] == '{}' ", v.as_str().unwrap()));
            module.push_str("{\n");
            module.push_str("\t\t\t\t\t\t\t\t\treturn token::Token::ENTITY(identifier)\n");
            module.push_str("\t\t\t\t\t\t\t\t}\n");
        }
        if k.as_str().unwrap() == "ENTITY_SUFFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if self.ch == '{}' ", v.as_str().unwrap()));
            module.push_str("{\n");
            module.push_str("\t\t\t\t\t\t\t\t\treturn token::Token::ENTITY(identifier)\n");
            module.push_str("\t\t\t\t\t\t\t\t}\n");
        }
    }

    module.push_str("\t\t\t\t\t\t\t\ttoken::Token::IDENT(identifier)\n\
                    \t\t\t\t\t\t\t}\n\
                \t\t\t\t\t\t}\n\
            \t\t\t\t\t} else if is_digit(self.ch) {\n\
                \t\t\t\t\t\tlet identifier: Vec<char> = read_number(self);\n\
                \t\t\t\t\t\ttoken::Token::INT(identifier)\n\
            \t\t\t\t\t} else if self.ch == '\"' {\n\
                \t\t\t\t\t\tlet str_value: Vec<char> = read_string(self);\n\
                \t\t\t\t\t\ttoken::Token::STRING(str_value)\n\
            \t\t\t\t\t} else {\n\
                \t\t\t\t\t\ttoken::Token::ILLEGAL\n\
            \t\t\t\t\t}\n\
    \t\t\t\t}\n");

    module.push_str("\t\t\t}\n");
    module.push_str("\t\tself.read_char();\n");
    module.push_str("\t\ttok\n");
    module.push_str("\t}\n");
    module.push_str("}\n");
    module.to_string()
}
