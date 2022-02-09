// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    CH(char),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    VAR(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "after" => Ok(Token::KEYWORD(identifier.to_vec())),
        "and" => Ok(Token::KEYWORD(identifier.to_vec())),
        "andalso" => Ok(Token::KEYWORD(identifier.to_vec())),
        "band" => Ok(Token::KEYWORD(identifier.to_vec())),
        "begin" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bnot" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bor" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bsl" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bsr" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bxor" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "cond" => Ok(Token::KEYWORD(identifier.to_vec())),
        "div" => Ok(Token::KEYWORD(identifier.to_vec())),
        "end" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fun" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "not" => Ok(Token::KEYWORD(identifier.to_vec())),
        "of" => Ok(Token::KEYWORD(identifier.to_vec())),
        "or" => Ok(Token::KEYWORD(identifier.to_vec())),
        "orelse" => Ok(Token::KEYWORD(identifier.to_vec())),
        "receive" => Ok(Token::KEYWORD(identifier.to_vec())),
        "rem" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "when" => Ok(Token::KEYWORD(identifier.to_vec())),
        "xor" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
