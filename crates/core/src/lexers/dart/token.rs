// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "null" | "true" | "false" | "String" | "Widget" | "Future" | "Object" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
        "abstract" | "else" | "import" | "super" | "as" | "enum" | "in" | "switch" | "assert"
        | "export" | "interface" | "sync" | "async" | "extends" | "is" | "this" | "await"
        | "extension" | "library" | "throw" | "external" | "mixin" | "case" | "factory" | "new"
        | "try" | "class" | "final" | "catch" | "typedef" | "on" | "var" | "const" | "finally"
        | "operator" | "void" | "continue" | "for" | "part" | "while" | "covariant"
        | "Function" | "rethrow" | "with" | "default" | "get" | "return" | "yield" | "deferred"
        | "hide" | "set" | "do" | "if" | "show" | "dynamic" | "implements" | "static" | "late"
        | "break" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
