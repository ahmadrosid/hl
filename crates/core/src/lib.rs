pub mod lexers;
use crate::lexers::*;

#[must_use]
pub fn render_html(input: Vec<char>, lang: &str) -> String {
    match lang {
        "actionscript" => actionscript::render::html(input),
        "bash" => bash::render::html(input),
        "c" => c::render::html(input),
        "clojure" => clojure::render::html(input),
        "css" => css::render::html(input),
        "cuda" => cuda::render::html(input),
        "dart" => dart::render::html(input),
        "edn" => edn::render::html(input),
        "erlang" => erlang::render::html(input),
        "cpp" => cpp::render::html(input),
        "cs" => cs::render::html(input),
        "go" => go::render::html(input),
        "groovy" => groovy::render::html(input),
        "haskell" => haskell::render::html(input),
        "html" => html::render::html(input),
        "java" => java::render::html(input),
        "javascript" => javascript::render::html(input),
        "json" => json::render::html(input),
        "lua" => lua::render::html(input),
        "makefile" => makefile::render::html(input),
        "markdown" => markdown::render::html(input),
        "nim" => nim::render::html(input),
        "php" => php::render::html(input),
        "python" => python::render::html(input),
        "raw" => raw::render::html(input),
        "ruby" => ruby::render::html(input),
        "rust" => rust::render::html(input),
        "toml" => toml::render::html(input),
        "typescript" => typescript::render::html(input),
        "vue" => vue::render::html(input),
        "yaml" => yaml::render::html(input),
        "zig" => zig::render::html(input),
        "kotlin" => kotlin::render::html(input),
        _ => String::new(),
    }
}
