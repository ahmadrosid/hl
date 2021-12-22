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
        token.push_str(",\n");
    }

    token.push_str("}\n\n");
    token.push_str("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {\n");
    token.push_str("\tlet identifiers: String = identifier.into_iter().collect();\n");
    token.push_str("\tmatch &identifiers[..] {\n");

    for (k, v) in get_constant(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in get_keyword(h) {
        token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("Ok(Token::{}),\n", k.as_str().unwrap()));
    }

    token.push_str("\t\t_ => Err(String::from(\"Not a keyword\"))\n");
    token.push_str("\t}\n");
    token.push_str("}\n");

    token
}

pub fn process_module(h: &Hash) -> String {
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
        \t\t};\n\
    \n\
        \t\tlet tok: token::Token;\n\
        \t\tmatch self.ch {\n\
");

    for (k, v) in get_base(h) {
        module.push_str(&format!("\t\t'{}' => ", v.as_str().unwrap()));
        module.push_str("{\n");
        module.push_str(&format!("\t\t\ttok = token::Token::{}(self.ch);\n", k.as_str().unwrap()));
        module.push_str("\t\t}\n");
    }

    module.push_str("\t\t_ => {\n\
            \t\t\treturn if is_letter(self.ch) {\n\
                \t\t\t\tlet identifier: Vec<char> = read_identifier(self);\n\
                \t\t\t\tmatch token::get_keyword_token(&identifier) {\n\
                    \t\t\t\t\t\tOk(keyword_token) => {\n\
                        \t\t\t\t\t\t\tkeyword_token\n\
                    \t\t\t\t\t\t},\n\
                    \t\t\t\t\t\tErr(_err) => {\n\
                        \t\t\t\t\t\t\ttoken::Token::IDENT(identifier)\n\
                    \t\t\t\t\t\t}\n\
                \t\t\t\t\t}\n\
            \t\t\t\t} else if is_digit(self.ch) {\n\
                \t\t\t\t\tlet identifier: Vec<char> = read_number(self);\n\
                \t\t\t\t\ttoken::Token::INT(identifier)\n\
            \t\t\t\t} else if self.ch == '\"' {\n\
                \t\t\t\t\tlet str_value: Vec<char> = read_string(self);\n\
                \t\t\t\t\ttoken::Token::STRING(str_value)\n\
            \t\t\t\t} else {\n\
                \t\t\t\t\ttoken::Token::ILLEGAL\n\
            \t\t\t\t}\n\
    \t\t\t}\n");

    module.push_str("\t\t}\n");
    module.push_str("\t\tself.read_char();\n");
    module.push_str("\t\ttok\n");
    module.push_str("\t}\n");
    module.push_str("}\n");
    module
}
