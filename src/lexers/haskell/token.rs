// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    INT(Vec<char>),
    IDENT(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "True" => Ok(Token::CONSTANT(identifier.to_vec())),
        "False" => Ok(Token::CONSTANT(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "type" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "of" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "then" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in" => Ok(Token::KEYWORD(identifier.to_vec())),
        "module" => Ok(Token::KEYWORD(identifier.to_vec())),
        "where" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
