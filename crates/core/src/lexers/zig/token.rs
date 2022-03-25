// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" | "false" | "null" | "undefined" | "self" => Ok(Token::CONSTANT(identifier.clone())),
        "const" | "pub" | "fn" | "try" | "struct" | "return" | "and" | "or" | "var" | "error"
        | "break" | "continue" | "unreachable" | "while" | "test" | "enum" | "union" | "async"
        | "await" | "suspend" | "resume" | "nosuspend" | "comptime" | "extern" | "export"
        | "threadlocal" | "if" | "orelse" | "catch" | "for" | "anytype" | "volatile" | "align"
        | "allowzero" | "packed" | "switch" | "inline" | "else" | "opaque" | "callconv"
        | "defer" | "errdefer" | "usingnamespace" | "asm" | "anyframe" | "noalias"
        | "linksection" | "void" | "bool" | "i64" | "u32" | "i32" | "f32" | "u8" | "anyerror"
        | "i8" | "i16" | "u16" | "u64" | "i128" | "u128" | "isize" | "usize" | "c_short"
        | "c_ushort" | "c_int" | "c_uint" | "c_long" | "c_ulong" | "c_longlong" | "c_ulonglong"
        | "c_longdouble" | "f16" | "f64" | "f128" | "anyopaque" | "noreturn" | "type"
        | "comptime_int" | "comptime_float" | "i7" | "u21" | "u4" | "u3" | "u2" | "i386" | "u0"
        | "i0" | "u29" | "u1" | "u24" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
