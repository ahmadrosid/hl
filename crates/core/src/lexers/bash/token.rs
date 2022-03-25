// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "builtin" => Ok(Token::CONSTANT(identifier.clone())),
        "command" => Ok(Token::CONSTANT(identifier.clone())),
        "compgen" => Ok(Token::CONSTANT(identifier.clone())),
        "echo" => Ok(Token::CONSTANT(identifier.clone())),
        "eval" => Ok(Token::CONSTANT(identifier.clone())),
        "exit" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "hash" => Ok(Token::CONSTANT(identifier.clone())),
        "kill" => Ok(Token::CONSTANT(identifier.clone())),
        "read" => Ok(Token::CONSTANT(identifier.clone())),
        "source" => Ok(Token::CONSTANT(identifier.clone())),
        "unset" => Ok(Token::CONSTANT(identifier.clone())),
        "test" => Ok(Token::CONSTANT(identifier.clone())),
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "printf" => Ok(Token::CONSTANT(identifier.clone())),
        "case" | "continue" | "do" | "done" | "elif" | "else" | "esac" | "export" | "fi"
        | "for" | "function" | "if" | "in" | "local" | "return" | "select" | "then" | "time"
        | "until" | "while" | "EOF" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
