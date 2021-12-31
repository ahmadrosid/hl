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
    CONSTANT(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "this" => Ok(Token::CONSTANT(identifier.to_vec())),
        "nullptr" => Ok(Token::CONSTANT(identifier.to_vec())),
        "NULL" => Ok(Token::CONSTANT(identifier.to_vec())),
        "size_t" => Ok(Token::CONSTANT(identifier.to_vec())),
        "auto" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "char" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "delete" => Ok(Token::KEYWORD(identifier.to_vec())),
        "double" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "float" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "inline" => Ok(Token::KEYWORD(identifier.to_vec())),
        "long" => Ok(Token::KEYWORD(identifier.to_vec())),
        "namespace" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "override" => Ok(Token::KEYWORD(identifier.to_vec())),
        "using" => Ok(Token::KEYWORD(identifier.to_vec())),
        "int" => Ok(Token::KEYWORD(identifier.to_vec())),
        "include" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endif" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "private" => Ok(Token::KEYWORD(identifier.to_vec())),
        "void" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static_cast" => Ok(Token::KEYWORD(identifier.to_vec())),
        "template" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword"))
    }
}
