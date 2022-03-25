// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "True" | "False" | "None" | "not" | "in" | "self" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "as" | "and" | "assert" | "async" | "await" | "break" | "class" | "continue" | "else"
        | "elif" | "except" | "def" | "del" | "finally" | "for" | "from" | "global" | "if"
        | "is" | "import" | "lambda" | "nonlocal" | "pass" | "print" | "raise" | "return"
        | "try" | "with" | "while" | "yield" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
