use clap::{arg, App, AppSettings};
mod generator;
mod lexers;
use crate::lexers::{rust, go, css, javascript};
use std::fs::read;

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax highlighting.")
        .arg(arg!([FILE_PATH] "File path to parse."))
        .arg(arg!(lang: -l [LANG] "Language."))
        .subcommand(
            App::new("generate")
                .short_flag('g')
                .long_flag("generate")
                .about("Generate lexer, this is for development only.")
                .arg(arg!([LEXER_PATH] "Lexer path"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let mut lang = "";
    let mut input: Vec<char> = vec!['0'];
    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let lexer_path = sub_matches.value_of("LEXER_PATH").expect("required");
            let s = generator::parse(lexer_path);
            println!("{}", s);
            std::process::exit(0x001);
        }
        _ => {
            if let Some(file) = matches.value_of("FILE_PATH") {
                let source = read(file).expect(&format!("Filed reading file {}", file));
                input = source.iter().map(|c| *c as char).collect::<Vec<_>>();
            }
            if let Some(language) = matches.value_of("lang") {
                lang = language;
            }
        }
    }

    match lang {
        "go" => {
            println!("{}", go::render::render_html(input));
        }
        "rust" => {
            println!("{}", rust::render::render_html(input));
        },
        "css" => {
            println!("{}", css::render::render_html(input));
        }
        "js" => {
            println!("{}", javascript::render::render_html(input));
        }
        _ => {
            println!("Language {} not supported", lang);
        }
    }
}
