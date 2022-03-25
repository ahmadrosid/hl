// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "null" | "this" => Ok(Token::CONSTANT(identifier.clone())),
        "abstract" | "as" | "async" | "await" | "base" | "bool" | "break" | "byte" | "case"
        | "catch" | "char" | "checked" | "class" | "const" | "continue" | "decimal" | "default"
        | "delegate" | "do" | "double" | "else" | "enum" | "event" | "explicit" | "extern"
        | "finally" | "fixed" | "float" | "for" | "foreach" | "goto" | "if" | "implicit" | "in"
        | "int" | "interface" | "internal" | "is" | "lock" | "long" | "namespace" | "new"
        | "object" | "operator" | "out" | "override" | "params" | "private" | "protected"
        | "public" | "readonly" | "ref" | "return" | "sbyte" | "sealed" | "short" | "sizeof"
        | "stackalloc" | "static" | "string" | "struct" | "switch" | "throw" | "try" | "typeof"
        | "uint" | "ulong" | "unchecked" | "unsafe" | "ushort" | "using" | "virtual" | "var"
        | "void" | "volatile" | "while" | "yield" | "add" | "remove" | "region" | "endregion"
        | "where" | "get" | "set" | "global" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
