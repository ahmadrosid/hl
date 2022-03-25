// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "undefined" | "null" | "number" | "string" | "boolean" | "type" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "Infinity" | "NaN" | "Math" | "Date" => Ok(Token::VAR(identifier.clone())),
        "async" | "await" | "break" | "as" | "switch" | "case" | "if" | "throw" | "else"
        | "var" | "get" | "module" | "instanceof" | "import" | "from" | "typeof" | "public"
        | "private" | "enum" | "export" | "finally" | "for" | "while" | "void" | "super"
        | "this" | "in" | "return" | "any" | "extends" | "static" | "let" | "package"
        | "implements" | "interface" | "function" | "new" | "try" | "yield" | "const"
        | "continue" | "do" | "catch" | "default" | "class" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
