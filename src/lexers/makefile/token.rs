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
        "PHONY" => Ok(Token::CONSTANT(identifier.to_vec())),
        "call" => Ok(Token::CONSTANT(identifier.to_vec())),
        "cd" => Ok(Token::CONSTANT(identifier.to_vec())),
        "command" => Ok(Token::CONSTANT(identifier.to_vec())),
        "test" => Ok(Token::CONSTANT(identifier.to_vec())),
        "echo" => Ok(Token::CONSTANT(identifier.to_vec())),
        "shell" => Ok(Token::CONSTANT(identifier.to_vec())),
        "MAKE" => Ok(Token::CONSTANT(identifier.to_vec())),
        "export" => Ok(Token::KEYWORD(identifier.to_vec())),
        "include" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
