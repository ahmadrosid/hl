// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    CH(char),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    VAR(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "undefined" => Ok(Token::CONSTANT(identifier.to_vec())),
        "self" => Ok(Token::CONSTANT(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "pub" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "struct" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "and" => Ok(Token::KEYWORD(identifier.to_vec())),
        "or" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "error" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "unreachable" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "test" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enum" => Ok(Token::KEYWORD(identifier.to_vec())),
        "union" => Ok(Token::KEYWORD(identifier.to_vec())),
        "async" => Ok(Token::KEYWORD(identifier.to_vec())),
        "await" => Ok(Token::KEYWORD(identifier.to_vec())),
        "suspend" => Ok(Token::KEYWORD(identifier.to_vec())),
        "resume" => Ok(Token::KEYWORD(identifier.to_vec())),
        "nosuspend" => Ok(Token::KEYWORD(identifier.to_vec())),
        "comptime" => Ok(Token::KEYWORD(identifier.to_vec())),
        "extern" => Ok(Token::KEYWORD(identifier.to_vec())),
        "export" => Ok(Token::KEYWORD(identifier.to_vec())),
        "threadlocal" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "orelse" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "anytype" => Ok(Token::KEYWORD(identifier.to_vec())),
        "volatile" => Ok(Token::KEYWORD(identifier.to_vec())),
        "align" => Ok(Token::KEYWORD(identifier.to_vec())),
        "allowzero" => Ok(Token::KEYWORD(identifier.to_vec())),
        "packed" => Ok(Token::KEYWORD(identifier.to_vec())),
        "switch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "inline" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "opaque" => Ok(Token::KEYWORD(identifier.to_vec())),
        "callconv" => Ok(Token::KEYWORD(identifier.to_vec())),
        "defer" => Ok(Token::KEYWORD(identifier.to_vec())),
        "errdefer" => Ok(Token::KEYWORD(identifier.to_vec())),
        "usingnamespace" => Ok(Token::KEYWORD(identifier.to_vec())),
        "asm" => Ok(Token::KEYWORD(identifier.to_vec())),
        "anyframe" => Ok(Token::KEYWORD(identifier.to_vec())),
        "noalias" => Ok(Token::KEYWORD(identifier.to_vec())),
        "linksection" => Ok(Token::KEYWORD(identifier.to_vec())),
        "void" => Ok(Token::KEYWORD(identifier.to_vec())),
        "bool" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i64" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u8" => Ok(Token::KEYWORD(identifier.to_vec())),
        "anyerror" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i8" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i16" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u16" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u64" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i128" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u128" => Ok(Token::KEYWORD(identifier.to_vec())),
        "isize" => Ok(Token::KEYWORD(identifier.to_vec())),
        "usize" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_short" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_ushort" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_int" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_uint" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_long" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_ulong" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_longlong" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_ulonglong" => Ok(Token::KEYWORD(identifier.to_vec())),
        "c_longdouble" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f16" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f64" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f128" => Ok(Token::KEYWORD(identifier.to_vec())),
        "anyopaque" => Ok(Token::KEYWORD(identifier.to_vec())),
        "noreturn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "type" => Ok(Token::KEYWORD(identifier.to_vec())),
        "comptime_int" => Ok(Token::KEYWORD(identifier.to_vec())),
        "comptime_float" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i7" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u21" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u4" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u3" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u2" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i386" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u0" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i0" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u29" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u1" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u24" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
