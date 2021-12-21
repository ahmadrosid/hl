use yaml_rust::yaml::{Yaml, Hash};

fn get_base(h: &Hash) -> &Hash {
    h.get(&Yaml::String("base".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_constant(h: &Hash) -> &Hash {
    h.get(&Yaml::String("constant".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_keyword(h: &Hash) -> &Hash {
    h.get(&Yaml::String("keyword".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}
pub fn process_token(h: &Hash) -> String {
    let mut token = String::new();
    token.push_str("#[derive(PartialEq)]\n");
    token.push_str("#[derive(Debug)]\n");
    token.push_str("pub enum Token {\n");
    token.push_str("\tILLEGAL,\n");
    token.push_str("\tEOF,\n");

    token.push_str("\n\t// Base\n");

    for (k, _v) in get_base(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(char),\n");
    }

    token.push_str("\n\t// Constants\n");
    for (k, _v) in get_constant(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    token.push_str("\n\t// Keyword\n");
    for (k, _v) in get_keyword(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    token.push_str("}\n\n");
    token.push_str("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {\n");
    token.push_str("\tlet identifiers: String = identifier.into_iter().collect();\n");
    token.push_str("\tmatch &identifiers[..] {\n");

    for (k, v) in get_keyword(h) {
        token.push_str("\t\t");
        token.push_str(&format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("OK(Token::{}),\n", k.as_str().unwrap()));
    }

    token.push_str("\t\t_ => Err(String::from(\"Not a keyword\"))\n");
    token.push_str("\t}\n");
    token.push_str("}\n");

    token
}

pub fn process_module(h: &Hash) -> String {
    let mut module = String::new();
    module.push_str("pub mod token;\n\n\
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
}\n\n\
pub fn read_char(&mut self) {\n\
    \tif self.read_position >= self.input.len() {\n\
        \t\tself.ch = '0';\n\
    \t} else {\n\
        \t\tself.ch = self.input[self.read_position];\n\
    \t}\n\
    \tself.position = self.read_position;\n\
    \tself.read_position = self.read_position + 1;\n\
}\n\
\n\
pub fn next_token(&mut self) -> token::Token {\n\
    \tlet read_identifier = |l: &mut Lexer| -> Vec<char> {\n\
        \t\tlet position = l.position;\n\
        \t\twhile l.position < l.input.len() && is_letter(l.ch) {\n\
            \t\t\tl.read_char();\n\
        \t\t}\n\
        \t\tl.input[position..l.position].to_vec()\n\
    \t};\n\
\n\
    \tlet read_string = |l: &mut Lexer| -> Vec<char> {\n\
        \t\tlet position = l.position;\n\
        \t\tl.read_char();\n\
        \t\twhile l.position < l.input.len() && l.ch != '\"' {\n\
            \t\t\tl.read_char()\n\
        \t\t}\n\
        \t\tl.read_char();\n\
        \t\tl.input[position..l.position].to_vec()\n\
    \t};\n\
\n\
    \tlet read_number = |l: &mut Lexer| -> Vec<char> {\n\
        \t\tlet position = l.position;\n\
        \t\twhile l.position < l.input.len() && is_digit(l.ch) {\n\
            \t\t\tl.read_char();\n\
        \t\t}\n\
        \t\tl.input[position..l.position].to_vec()\n\
    \t};\n\
\n\
    \tlet tok: token::Token;\n\
    \tmatch self.ch {\n\
");

    for (k, v) in get_base(h) {
        module.push_str(&format!("\t\t'{}' => ", v.as_str().unwrap()));
        module.push_str("{\n");
        module.push_str(&format!("\t\t\ttok = token::Token::{}(self.ch);\n", k.as_str().unwrap()));
        module.push_str("\t\t},\n");
    }

    module.push_str("\t}\n");
    module.push_str("}\n");
    module
}
