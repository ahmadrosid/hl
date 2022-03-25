// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "null" | "true" | "false" | "this" | "Any" | "Array" | "Boolean" | "Double" | "Long"
        | "Int" | "String" | "Short" | "Lazy" | "List" => Ok(Token::CONSTANT(identifier.clone())),
        "as" | "abstract" | "break" | "class" | "continue" | "companion" | "data" | "do"
        | "else" | "enum" | "external" | "expect" | "for" | "fun" | "if" | "in" | "internal"
        | "import" | "interface" | "inline" | "is" | "noinline" | "object" | "open"
        | "operator" | "override" | "package" | "public" | "private" | "return" | "super"
        | "set" | "get" | "throw" | "try" | "typealias" | "typeof" | "val" | "var" | "when"
        | "while" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
