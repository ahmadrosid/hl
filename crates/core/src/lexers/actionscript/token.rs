// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "null" => Ok(Token::CONSTANT(identifier.clone())),
        "this" => Ok(Token::CONSTANT(identifier.clone())),
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "Math" => Ok(Token::CONSTANT(identifier.clone())),
        "uint" | "int" | "break" | "case" | "continue" | "default" | "do" | "else" | "for"
        | "if" | "in" | "import" | "label" | "return" | "super" | "switch" | "throw" | "try"
        | "while" | "with" | "dynamic" | "final" | "internal" | "native" | "override"
        | "private" | "protected" | "public" | "static" | "class" | "const" | "extends"
        | "function" | "get" | "implements" | "interface" | "namespace" | "package" | "set"
        | "var" | "new" | "void" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
