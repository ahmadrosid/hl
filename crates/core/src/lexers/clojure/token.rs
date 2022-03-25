// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "nil" | "full" => Ok(Token::CONSTANT(identifier.clone())),
        "binding" | "case" | "catch" | "cond" | "do" | "ns" | "def" | "defonce" | "defmulti"
        | "defmethod" | "defn" | "if" | "fn" | "require" | "when" | "try" | "throw" | "for"
        | "let" | "defn-" | "in-ns" | "if-let" | "s/defn" | "if-not" | "when-not" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
