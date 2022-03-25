// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" => Ok(Token::CONSTANT(identifier.clone())),
        "Rails" => Ok(Token::VAR(identifier.clone())),
        "alias" | "and" | "BEGIN" | "begin" | "break" | "case" | "class" | "def" | "defined?"
        | "do" | "else" | "elsif" | "END" | "end" | "ensure" | "for" | "if" | "in" | "module"
        | "next" | "nil" | "not" | "or" | "private" | "redo" | "rescue" | "retry" | "return"
        | "self" | "super" | "then" | "undef" | "unless" | "until" | "when" | "while" | "yield" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
