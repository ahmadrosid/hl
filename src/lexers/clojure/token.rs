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
        "nil" => Ok(Token::CONSTANT(identifier.to_vec())),
        "full" => Ok(Token::CONSTANT(identifier.to_vec())),
        "binding" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "cond" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "ns" => Ok(Token::KEYWORD(identifier.to_vec())),
        "def" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defonce" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defmulti" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defmethod" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "require" => Ok(Token::KEYWORD(identifier.to_vec())),
        "when" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defn-" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in-ns" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if-let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "s/defn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if-not" => Ok(Token::KEYWORD(identifier.to_vec())),
        "when-not" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
