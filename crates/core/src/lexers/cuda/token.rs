// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "NULL" | "size_t" => Ok(Token::CONSTANT(identifier.clone())),
        "asm" | "const" | "char" | "case" | "catch" | "continue" | "do" | "default" | "define"
        | "delete" | "double" | "else" | "extern" | "enum" | "float" | "goto" | "for" | "if"
        | "ifdef" | "ifndef" | "endif" | "long" | "return" | "int" | "include" | "inline"
        | "namespace" | "unsigned" | "union" | "using" | "break" | "void" | "volatile"
        | "register" | "signed" | "static" | "struct" | "sizeof" | "short" | "switch"
        | "typedef" | "try" | "throw" | "while" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
