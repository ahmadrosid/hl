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
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        "Args" => Ok(Token::CONSTANT(identifier.to_vec())),
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "nil" => Ok(Token::CONSTANT(identifier.to_vec())),
        "bool" => Ok(Token::CONSTANT(identifier.to_vec())),
        "int" => Ok(Token::CONSTANT(identifier.to_vec())),
        "uint" => Ok(Token::CONSTANT(identifier.to_vec())),
        "byte" => Ok(Token::CONSTANT(identifier.to_vec())),
        "string" => Ok(Token::CONSTANT(identifier.to_vec())),
        "error" => Ok(Token::CONSTANT(identifier.to_vec())),
        "rune" => Ok(Token::CONSTANT(identifier.to_vec())),
        "uintptr" => Ok(Token::CONSTANT(identifier.to_vec())),
        "float" => Ok(Token::CONSTANT(identifier.to_vec())),
        "package" => Ok(Token::KEYWORD(identifier.to_vec())),
        "func" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "switch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "default" => Ok(Token::KEYWORD(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "type" => Ok(Token::KEYWORD(identifier.to_vec())),
        "interface" => Ok(Token::KEYWORD(identifier.to_vec())),
        "chan" => Ok(Token::KEYWORD(identifier.to_vec())),
        "struct" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "range" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "map" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword"))
    }
}
