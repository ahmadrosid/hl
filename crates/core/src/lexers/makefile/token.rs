// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "PHONY" => Ok(Token::CONSTANT(identifier.clone())),
        "call" => Ok(Token::CONSTANT(identifier.clone())),
        "cd" => Ok(Token::CONSTANT(identifier.clone())),
        "command" => Ok(Token::CONSTANT(identifier.clone())),
        "test" => Ok(Token::CONSTANT(identifier.clone())),
        "echo" => Ok(Token::CONSTANT(identifier.clone())),
        "shell" => Ok(Token::CONSTANT(identifier.clone())),
        "MAKE" => Ok(Token::CONSTANT(identifier.clone())),
        "export" => Ok(Token::KEYWORD(identifier.clone())),
        "include" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
