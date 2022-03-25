// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "nil" => Ok(Token::CONSTANT(identifier.clone())),
        "and" | "break" | "do" | "else" | "elseif" | "end" | "for" | "function" | "if" | "in"
        | "local" | "not" | "or" | "repeat" | "return" | "then" | "while" | "until" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
