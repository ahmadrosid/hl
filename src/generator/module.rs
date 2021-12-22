use yaml_rust::yaml::Hash;
use crate::generator;
use crate::generator::get_prefix;

pub fn generate_module(h: &Hash) -> String {
    let mut module = String::new();
    module.push_str("pub mod token;\n\
pub mod render;\n\n\
pub struct Lexer {\n\
    \tinput: Vec<char>,\n\
    \tpub position: usize,\n\
    \tpub read_position: usize,\n\
    \tpub ch: char\n\
}\n\
\n\
fn is_letter(ch: char) -> bool {\n\
    \t'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'\n\
}\n\
\n\
fn is_digit(ch: char) -> bool {\n\
    \t'0' <= ch && ch <= '9'\n\
}\n\
\n\
impl Lexer {\n\
    \tpub fn new(input: Vec<char>) -> Self {\n\
        \t\tSelf {\n\
        \t\t\tinput,\n\
        \t\t\tposition: 0,\n\
        \t\t\tread_position: 0,\n\
        \t\t\tch: '0'\n\
        \t\t}\n\
    \t}\n\
    \tpub fn read_char(&mut self) {\n\
        \t\tif self.read_position >= self.input.len() {\n\
            \t\t\tself.ch = '0';\n\
        \t\t} else {\n\
            \t\t\tself.ch = self.input[self.read_position];\n\
        \t\t}\n\
        \t\tself.position = self.read_position;\n\
        \t\tself.read_position = self.read_position + 1;\n\
    \t}\n\
    \n\
    \tpub fn next_token(&mut self) -> token::Token {\n\
        \t\tlet read_identifier = |l: &mut Lexer| -> Vec<char> {\n\
            \t\t\tlet position = l.position;\n\
            \t\t\twhile l.position < l.input.len() && is_letter(l.ch) {\n\
                \t\t\t\tl.read_char();\n\
            \t\t\t}\n\
            \t\t\tl.input[position..l.position].to_vec()\n\
        \t\t};\n\
    \n\
        \t\tlet read_string = |l: &mut Lexer| -> Vec<char> {\n\
            \t\t\tlet position = l.position;\n\
            \t\t\tl.read_char();\n\
            \t\t\twhile l.position < l.input.len() && l.ch != '\"' {\n\
                \t\t\t\tl.read_char()\n\
            \t\t\t}\n\
            \t\t\tl.read_char();\n\
            \t\t\tl.input[position..l.position].to_vec()\n\
        \t\t};\n\
    \n\
        \t\tlet read_number = |l: &mut Lexer| -> Vec<char> {\n\
            \t\t\tlet position = l.position;\n\
            \t\t\twhile l.position < l.input.len() && is_digit(l.ch) {\n\
                \t\t\t\tl.read_char();\n\
            \t\t\t}\n\
            \t\t\tl.input[position..l.position].to_vec()\n\
        \t\t};\n\n\
    ");

    if generator::slash_comment_enable(h) {
        module.push_str("\t\tlet read_comment = |l: &mut Lexer| -> Vec<char> {\n\
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

    module.push_str("\t\tlet tok: token::Token;\n\
        \t\tmatch self.ch {\n\
    ");

    for (k, v) in generator::get_base(h) {
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
    module.push_str("\t\t\t\t\ttok = token::Token::CH(self.ch);\n");
    module.push_str("\t\t\t\t} else {\n");
    module.push_str("\t\t\t\t\ttok = token::Token::EOF;\n");
    module.push_str("\t\t\t\t}\n");
    module.push_str("\t\t\t}\n");

    if generator::slash_comment_enable(h) {
        module.push_str("\t\t\t'/' => {\n");
        module.push_str("\t\t\t\tif self.input[self.position+1] == '/' {\n");
        module.push_str("\t\t\t\t\ttok = token::Token::COMMENT(read_comment(self));\n");
        module.push_str("\t\t\t\t} else {\n");
        module.push_str("\t\t\t\t\ttok = token::Token::ENDL(self.ch);\n");
        module.push_str("\t\t\t\t}\n");
        module.push_str("\t\t\t}\n");
    }

    module.push_str("\t\t\t_ => {\n\
            \t\t\t\treturn if is_letter(self.ch) {\n\
                \t\t\t\t\tlet prev_pos = self.position;\n\
                \t\t\t\t\tlet identifier: Vec<char> = read_identifier(self);\n\
                \t\t\t\t\tmatch token::get_keyword_token(&identifier) {\n\
                    \t\t\t\t\t\t\tOk(keyword_token) => {\n\
    ");

    for (k, v) in get_prefix(h) {
        if k.as_str().unwrap() == "ENTITY_TOKEN_SUFFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if self.ch == '{}' ", v.as_str().unwrap()));
            module.push_str("{\n\t\t\t\t\t\t\t\t\t");
            module.push_str("self.read_char();\n\t\t\t\t\t\t\t\t\t");
            module.push_str("return token::Token::ENTITY(self.input[prev_pos..self.position].to_vec());\n");
            module.push_str("\t\t\t\t\t\t\t\t}\n");
        }
    }

    module.push_str("\t\t\t\t\t\t\t\tkeyword_token\n\
                    \t\t\t\t\t\t\t},\n\
                    \t\t\t\t\t\t\tErr(_err) => {\n\
    ");

    for (k, v) in get_prefix(h) {
        if k.as_str().unwrap() == "ENTITY_PREFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if self.input[prev_pos-1] == '{}' ", v.as_str().unwrap()));
            module.push_str("{\n");
            module.push_str("\t\t\t\t\t\t\t\t\treturn token::Token::ENTITY(identifier)\n");
            module.push_str("\t\t\t\t\t\t\t\t}\n");
        }
        if k.as_str().unwrap() == "ENTITY_SUFFIX" {
            module.push_str("\t\t\t\t\t\t\t\t");
            module.push_str(&format!("if self.input[self.position] == '{}' ", v.as_str().unwrap()));
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
    module
}
