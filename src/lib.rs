pub mod color;
pub mod generator;
pub mod lexers;
use crate::lexers::*;

#[cfg(test)]
mod test;

pub fn render_html(input: Vec<char>, lang: &str) -> String {
    return match lang {
        "bash" => bash::render::render_html(input),
        "c" => c::render::render_html(input),
        "clojure" | "clj" => clojure::render::render_html(input),
        "css" => css::render::render_html(input),
        "cuda" => cuda::render::render_html(input),
        "edn" => edn::render::render_html(input),
        "go" => go::render::render_html(input),
        "groovy" => groovy::render::render_html(input),
        "hs" | "haskell" => haskell::render::render_html(input),
        "html" => html::render::render_html(input),
        "cpp" => cpp::render::render_html(input),
        "cs" | "c#" => cs::render::render_html(input),
        "java" => java::render::render_html(input),
        "js" | "javascript" => javascript::render::render_html(input),
        "json" => json::render::render_html(input),
        "lua" => lua::render::render_html(input),
        "Makefile" => makefile::render::render_html(input),
        "markdown" | "md" => markdown::render::render_html(input),
        "php" => php::render::render_html(input),
        "python" => python::render::render_html(input),
        "rust" => rust::render::render_html(input),
        "toml" => toml::render::render_html(input),
        "ts" | "typescript" => typescript::render::render_html(input),
        "yaml" | "yml" => yaml::render::render_html(input),
        _ => raw::render::render_html(input),
    };
}
