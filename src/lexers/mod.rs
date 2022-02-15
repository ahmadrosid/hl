pub mod actionscript;
pub mod bash;
pub mod c;
pub mod clojure;
pub mod cpp;
pub mod cs;
pub mod css;
pub mod cuda;
pub mod dart;
pub mod edn;
pub mod erlang;
pub mod go;
pub mod groovy;
pub mod haskell;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod kotlin;
pub mod lua;
pub mod makefile;
pub mod markdown;
pub mod nim;
pub mod php;
pub mod python;
pub mod raw;
pub mod ruby;
pub mod rust;
pub mod toml;
pub mod typescript;
pub mod vue;
pub mod yaml;
pub mod zig;

#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    STRING(Vec<char>),
    ENTITYTAG(Vec<char>),
}
