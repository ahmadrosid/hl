// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "gem" => Ok(Token::ENTITY(identifier.to_vec())),
        "source" => Ok(Token::ENTITY(identifier.to_vec())),
        "group" => Ok(Token::ENTITY(identifier.to_vec())),
        "install_if" => Ok(Token::ENTITY(identifier.to_vec())),
        "gemspec" => Ok(Token::ENTITY(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "end" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
