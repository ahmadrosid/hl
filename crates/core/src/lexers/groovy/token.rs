// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "false" | "null" | "true" => Ok(Token::CONSTANT(identifier.clone())),
        "as" | "assert" | "break" | "case" | "catch" | "class" | "const" | "continue" | "def"
        | "default" | "do" | "else" | "enum" | "extends" | "final" | "finally" | "for" | "goto"
        | "if" | "implements" | "import" | "in" | "instanceof" | "interface" | "new"
        | "package" | "private" | "public" | "return" | "super" | "static" | "switch" | "this"
        | "throw" | "throws" | "trait" | "try" | "var" | "while" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
