// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "Args" | "true" | "false" | "nil" | "bool" | "int" | "uint" | "byte" | "string"
        | "rune" | "uintptr" | "float" | "float32" | "float64" | "int8" | "int16" | "int32"
        | "int64" | "uint8" | "uint16" | "uint32" | "uint64" | "complex" | "complex64"
        | "complex128" => Ok(Token::CONSTANT(identifier.clone())),
        "break" | "case" | "chan" | "const" | "continue" | "default" | "defer" | "else"
        | "error" | "fallthrough" | "for" | "func" | "go" | "goto" | "if" | "import"
        | "interface" | "map" | "package" | "range" | "return" | "select" | "struct" | "switch"
        | "type" | "var" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
