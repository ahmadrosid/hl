// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "PHONY" | "call" | "cd" | "command" | "test" | "echo" | "shell" | "MAKE" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "export" | "include" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
