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
        "True" => Ok(Token::CONSTANT(identifier.to_vec())),
        "False" => Ok(Token::CONSTANT(identifier.to_vec())),
        "None" => Ok(Token::CONSTANT(identifier.to_vec())),
        "not" => Ok(Token::CONSTANT(identifier.to_vec())),
        "in" => Ok(Token::CONSTANT(identifier.to_vec())),
        "as" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "except" => Ok(Token::KEYWORD(identifier.to_vec())),
        "def" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "from" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "pass" => Ok(Token::KEYWORD(identifier.to_vec())),
        "raise" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
