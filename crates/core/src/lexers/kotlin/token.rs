// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "null" => Ok(Token::CONSTANT(identifier.clone())),
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "this" => Ok(Token::CONSTANT(identifier.clone())),
        "Any" => Ok(Token::CONSTANT(identifier.clone())),
        "Array" => Ok(Token::CONSTANT(identifier.clone())),
        "Boolean" => Ok(Token::CONSTANT(identifier.clone())),
        "Double" => Ok(Token::CONSTANT(identifier.clone())),
        "Long" => Ok(Token::CONSTANT(identifier.clone())),
        "Int" => Ok(Token::CONSTANT(identifier.clone())),
        "String" => Ok(Token::CONSTANT(identifier.clone())),
        "Short" => Ok(Token::CONSTANT(identifier.clone())),
        "Lazy" => Ok(Token::CONSTANT(identifier.clone())),
        "List" => Ok(Token::CONSTANT(identifier.clone())),
        "as" | "abstract" | "break" | "class" | "continue" | "companion" | "data" | "do"
        | "else" | "enum" | "external" | "expect" | "for" | "fun" | "if" | "in" | "internal"
        | "import" | "interface" | "inline" | "is" | "noinline" | "object" | "open"
        | "operator" | "override" | "package" | "public" | "private" | "return" | "super"
        | "set" | "get" | "throw" | "try" | "typealias" | "typeof" | "val" | "var" | "when"
        | "while" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
