// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "Infinity" => Ok(Token::VAR(identifier.clone())),
        "NaN" => Ok(Token::VAR(identifier.clone())),
        "Math" => Ok(Token::VAR(identifier.clone())),
        "Date" => Ok(Token::VAR(identifier.clone())),
        "true" | "false" | "undefined" | "null" => Ok(Token::CONSTANT(identifier.clone())),
        "async" | "await" | "break" | "case" | "catch" | "class" | "const" | "continue"
        | "debugger" | "default" | "delete" | "do" | "else" | "enum" | "export" | "extends"
        | "finally" | "for" | "function" | "if" | "implements" | "import" | "in" | "instanceof"
        | "interface" | "let" | "new" | "package" | "private" | "protected" | "public"
        | "return" | "super" | "switch" | "static" | "this" | "throw" | "try" | "typeof"
        | "var" | "void" | "while" | "with" | "yield" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
