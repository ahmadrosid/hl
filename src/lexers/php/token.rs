// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    CH(char),
    ENDL(char),
    INT(Vec<char>),
    IDENT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    KEYWORD(Vec<char>),
    ENTITYTAG(Vec<char>),
    THIS(Vec<char>),
    TRUE(Vec<char>),
    FALSE(Vec<char>),
    NULL(Vec<char>),
    CSTRING(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        "this" => Ok(Token::THIS(identifier.to_vec())),
        "true" => Ok(Token::TRUE(identifier.to_vec())),
        "false" => Ok(Token::FALSE(identifier.to_vec())),
        "null" => Ok(Token::NULL(identifier.to_vec())),
        "String" => Ok(Token::CSTRING(identifier.to_vec())),
        "require_once" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword"))
    }
}
