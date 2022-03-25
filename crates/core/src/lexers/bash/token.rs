// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "builtin" | "command" | "compgen" | "echo" | "eval" | "exit" | "false" | "hash"
        | "kill" | "read" | "source" | "unset" | "test" | "true" | "printf" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "case" | "continue" | "do" | "done" | "elif" | "else" | "esac" | "export" | "fi"
        | "for" | "function" | "if" | "in" | "local" | "return" | "select" | "then" | "time"
        | "until" | "while" | "EOF" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
