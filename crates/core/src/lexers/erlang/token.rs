// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "after" | "and" | "andalso" | "band" | "begin" | "bnot" | "bor" | "bsl" | "bsr"
        | "bxor" | "case" | "catch" | "cond" | "div" | "end" | "fun" | "if" | "let" | "not"
        | "of" | "or" | "orelse" | "receive" | "rem" | "try" | "when" | "xor" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
