// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "nil" | "true" | "false" => Ok(Token::CONSTANT(identifier.clone())),
        "addr" | "and" | "as" | "asm" | "bind" | "block" | "break" | "case" | "cast"
        | "concept" | "const" | "continue" | "converter" | "defer" | "discard" | "distinct"
        | "div" | "do" | "elif" | "else" | "echo" | "end" | "enum" | "except" | "export"
        | "finally" | "for" | "from" | "func" | "if" | "import" | "in" | "include"
        | "interface" | "is" | "isnot" | "iterator" | "let" | "macro" | "method" | "mixin"
        | "mod" | "not" | "notin" | "object" | "of" | "or" | "out" | "proc" | "ptr" | "raise"
        | "ref" | "return" | "shl" | "shr" | "static" | "template" | "try" | "tuple" | "type"
        | "using" | "var" | "when" | "while" | "yield" | "xor" | "int" | "int8" | "int16"
        | "int32" | "int64" | "uint" | "uint8" | "uint16" | "uint32" | "uint64" | "float"
        | "float32" | "float64" | "char" | "string" | "cstring" | "bool" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
