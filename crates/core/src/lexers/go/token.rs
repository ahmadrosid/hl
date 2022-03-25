// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "Args" => Ok(Token::CONSTANT(identifier.clone())),
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "nil" => Ok(Token::CONSTANT(identifier.clone())),
        "bool" => Ok(Token::CONSTANT(identifier.clone())),
        "int" => Ok(Token::CONSTANT(identifier.clone())),
        "uint" => Ok(Token::CONSTANT(identifier.clone())),
        "byte" => Ok(Token::CONSTANT(identifier.clone())),
        "string" => Ok(Token::CONSTANT(identifier.clone())),
        "rune" => Ok(Token::CONSTANT(identifier.clone())),
        "uintptr" => Ok(Token::CONSTANT(identifier.clone())),
        "float" => Ok(Token::CONSTANT(identifier.clone())),
        "float32" => Ok(Token::CONSTANT(identifier.clone())),
        "float64" => Ok(Token::CONSTANT(identifier.clone())),
        "int8" => Ok(Token::CONSTANT(identifier.clone())),
        "int16" => Ok(Token::CONSTANT(identifier.clone())),
        "int32" => Ok(Token::CONSTANT(identifier.clone())),
        "int64" => Ok(Token::CONSTANT(identifier.clone())),
        "uint8" => Ok(Token::CONSTANT(identifier.clone())),
        "uint16" => Ok(Token::CONSTANT(identifier.clone())),
        "uint32" => Ok(Token::CONSTANT(identifier.clone())),
        "uint64" => Ok(Token::CONSTANT(identifier.clone())),
        "complex" => Ok(Token::CONSTANT(identifier.clone())),
        "complex64" => Ok(Token::CONSTANT(identifier.clone())),
        "complex128" => Ok(Token::CONSTANT(identifier.clone())),
        "break" | "case" | "chan" | "const" | "continue" | "default" | "defer" | "else"
        | "error" | "fallthrough" | "for" | "func" | "go" | "goto" | "if" | "import"
        | "interface" | "map" | "package" | "range" | "return" | "select" | "struct" | "switch"
        | "type" | "var" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
