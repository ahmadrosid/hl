// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "after" => Ok(Token::KEYWORD(identifier.clone())),
        "and" => Ok(Token::KEYWORD(identifier.clone())),
        "andalso" => Ok(Token::KEYWORD(identifier.clone())),
        "band" => Ok(Token::KEYWORD(identifier.clone())),
        "begin" => Ok(Token::KEYWORD(identifier.clone())),
        "bnot" => Ok(Token::KEYWORD(identifier.clone())),
        "bor" => Ok(Token::KEYWORD(identifier.clone())),
        "bsl" => Ok(Token::KEYWORD(identifier.clone())),
        "bsr" => Ok(Token::KEYWORD(identifier.clone())),
        "bxor" => Ok(Token::KEYWORD(identifier.clone())),
        "case" => Ok(Token::KEYWORD(identifier.clone())),
        "catch" => Ok(Token::KEYWORD(identifier.clone())),
        "cond" => Ok(Token::KEYWORD(identifier.clone())),
        "div" => Ok(Token::KEYWORD(identifier.clone())),
        "end" => Ok(Token::KEYWORD(identifier.clone())),
        "fun" => Ok(Token::KEYWORD(identifier.clone())),
        "if" => Ok(Token::KEYWORD(identifier.clone())),
        "let" => Ok(Token::KEYWORD(identifier.clone())),
        "not" => Ok(Token::KEYWORD(identifier.clone())),
        "of" => Ok(Token::KEYWORD(identifier.clone())),
        "or" => Ok(Token::KEYWORD(identifier.clone())),
        "orelse" => Ok(Token::KEYWORD(identifier.clone())),
        "receive" => Ok(Token::KEYWORD(identifier.clone())),
        "rem" => Ok(Token::KEYWORD(identifier.clone())),
        "try" => Ok(Token::KEYWORD(identifier.clone())),
        "when" => Ok(Token::KEYWORD(identifier.clone())),
        "xor" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
