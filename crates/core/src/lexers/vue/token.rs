// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "template" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "script" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "style" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
