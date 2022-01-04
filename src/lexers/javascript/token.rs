// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
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
    VINFINITY(Vec<char>),
    VNAN(Vec<char>),
    MATH(Vec<char>),
    DATE(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "undefined" => Ok(Token::CONSTANT(identifier.to_vec())),
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Infinity" => Ok(Token::VINFINITY(identifier.to_vec())),
        "NaN" => Ok(Token::VNAN(identifier.to_vec())),
        "Math" => Ok(Token::MATH(identifier.to_vec())),
        "Date" => Ok(Token::DATE(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "as" => Ok(Token::KEYWORD(identifier.to_vec())),
        "new" => Ok(Token::KEYWORD(identifier.to_vec())),
        "this" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "function" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "void" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
