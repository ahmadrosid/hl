// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    INT(Vec<char>),
    IDENT(Vec<char>),
    CH(char),
    ENTITY(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "using" => Ok(Token::KEYWORD(identifier.to_vec())),
        "namespace" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "string" => Ok(Token::KEYWORD(identifier.to_vec())),
        "decimal" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "new" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bool" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
