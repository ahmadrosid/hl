// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "nil" => Ok(Token::CONSTANT(identifier.clone())),
        "full" => Ok(Token::CONSTANT(identifier.clone())),
        "binding" => Ok(Token::KEYWORD(identifier.clone())),
        "case" => Ok(Token::KEYWORD(identifier.clone())),
        "catch" => Ok(Token::KEYWORD(identifier.clone())),
        "cond" => Ok(Token::KEYWORD(identifier.clone())),
        "do" => Ok(Token::KEYWORD(identifier.clone())),
        "ns" => Ok(Token::KEYWORD(identifier.clone())),
        "def" => Ok(Token::KEYWORD(identifier.clone())),
        "defonce" => Ok(Token::KEYWORD(identifier.clone())),
        "defmulti" => Ok(Token::KEYWORD(identifier.clone())),
        "defmethod" => Ok(Token::KEYWORD(identifier.clone())),
        "defn" => Ok(Token::KEYWORD(identifier.clone())),
        "if" => Ok(Token::KEYWORD(identifier.clone())),
        "fn" => Ok(Token::KEYWORD(identifier.clone())),
        "require" => Ok(Token::KEYWORD(identifier.clone())),
        "when" => Ok(Token::KEYWORD(identifier.clone())),
        "try" => Ok(Token::KEYWORD(identifier.clone())),
        "throw" => Ok(Token::KEYWORD(identifier.clone())),
        "for" => Ok(Token::KEYWORD(identifier.clone())),
        "let" => Ok(Token::KEYWORD(identifier.clone())),
        "defn-" => Ok(Token::KEYWORD(identifier.clone())),
        "in-ns" => Ok(Token::KEYWORD(identifier.clone())),
        "if-let" => Ok(Token::KEYWORD(identifier.clone())),
        "s/defn" => Ok(Token::KEYWORD(identifier.clone())),
        "if-not" => Ok(Token::KEYWORD(identifier.clone())),
        "when-not" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
