// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "this" | "nullptr" | "NULL" | "size_t" | "int64_t" | "uint32_t" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "asm" | "auto" | "bool" | "break" | "const" | "class" | "char" | "catch" | "constexpr"
        | "continue" | "default" | "define" | "delete" | "do" | "double" | "else" | "enum"
        | "extern" | "explicit" | "float" | "final" | "friend" | "for" | "if" | "inline"
        | "int" | "long" | "namespace" | "new" | "noexcept" | "return" | "override"
        | "operator" | "include" | "endif" | "public" | "private" | "protected" | "pragma"
        | "short" | "signed" | "sizeof" | "static" | "static_cast" | "struct" | "switch"
        | "template" | "typedef" | "typename" | "try" | "throw" | "using" | "union"
        | "unsigned" | "void" | "virtual" | "volatile" | "while" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
