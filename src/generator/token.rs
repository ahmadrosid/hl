use yaml_rust::yaml::Hash;
use crate::generator;

pub fn generate_token(h: &Hash) -> String {
    let mut token = String::new();
    token.push_str("#[derive(PartialEq)]\n");
    token.push_str("#[derive(Debug)]\n");
    token.push_str("pub enum Token {\n");
    token.push_str("\tILLEGAL,\n");
    token.push_str("\tEOF,\n");
    token.push_str("\tCH(char),\n");
    token.push_str("\tENDL(char),\n");

    token.push_str("\n\t// Base\n");
    for (k, _v) in generator::get_base(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(char),\n");
    }

    token.push_str("\n\t// Constants\n");
    for (k, _v) in generator::get_constant(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    token.push_str("\n\t// Skip token\n");
    for (k, _v) in generator::get_skip(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    token.push_str("\n\t// Entity\n");
    for (k, _v) in generator::get_entity(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    token.push_str("\n\t// Keyword\n");
    for (k, _v) in generator::get_keyword(h) {
        token.push_str("\t");
        token.push_str(k.as_str().unwrap());
        token.push_str("(Vec<char>),\n");
    }

    if generator::slash_comment_enable(h) {
        token.push_str("\tCOMMENT(Vec<char>),\n");
    }

    token.push_str("}\n\n");
    token.push_str("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {\n");
    token.push_str("\tlet identifiers: String = identifier.into_iter().collect();\n");
    token.push_str("\tmatch &identifiers[..] {\n");

    for (k, v) in generator::get_constant(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in generator::get_keyword(h) {
        token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
    }

    token.push_str("\t\t_ => Err(String::from(\"Not a keyword\"))\n");
    token.push_str("\t}\n");
    token.push_str("}\n");

    token
}
