// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    INT(Vec<char>),
    IDENT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "nil" => Ok(Token::CONSTANT(identifier.to_vec())),
        "full" => Ok(Token::CONSTANT(identifier.to_vec())),
        "ns" => Ok(Token::KEYWORD(identifier.to_vec())),
        "def" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "when" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
