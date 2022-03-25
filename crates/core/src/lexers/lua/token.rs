// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "nil" => Ok(Token::CONSTANT(identifier.clone())),
        "and" => Ok(Token::KEYWORD(identifier.clone())),
        "break" => Ok(Token::KEYWORD(identifier.clone())),
        "do" => Ok(Token::KEYWORD(identifier.clone())),
        "else" => Ok(Token::KEYWORD(identifier.clone())),
        "elseif" => Ok(Token::KEYWORD(identifier.clone())),
        "end" => Ok(Token::KEYWORD(identifier.clone())),
        "for" => Ok(Token::KEYWORD(identifier.clone())),
        "function" => Ok(Token::KEYWORD(identifier.clone())),
        "if" => Ok(Token::KEYWORD(identifier.clone())),
        "in" => Ok(Token::KEYWORD(identifier.clone())),
        "local" => Ok(Token::KEYWORD(identifier.clone())),
        "not" => Ok(Token::KEYWORD(identifier.clone())),
        "or" => Ok(Token::KEYWORD(identifier.clone())),
        "repeat" => Ok(Token::KEYWORD(identifier.clone())),
        "return" => Ok(Token::KEYWORD(identifier.clone())),
        "then" => Ok(Token::KEYWORD(identifier.clone())),
        "while" => Ok(Token::KEYWORD(identifier.clone())),
        "until" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
