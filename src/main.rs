use clap::{arg, App, AppSettings};
mod generator;
mod lexers;
use crate::lexers::{rust, go};

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax highlighting.")
        .arg(arg!([FILE_PATH] "File path to parse.").required(true))
        .arg(arg!(lang: -l [LANG] "Language.").required(true))
        .subcommand(
            App::new("generate")
                .short_flag('g')
                .long_flag("generate")
                .about("Generate lexer.")
                .arg(arg!([LEXER_PATH] "Lexer path"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let mut file_path = "";
    let mut lang = "";
    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let lexer_path = sub_matches.value_of("LEXER_PATH").expect("required");
            let s = generator::parse(lexer_path);
            println!("{}", s);
        }
        _ => {
            if let Some(file) = matches.value_of("FILE_PATH") {
                file_path = file;
            }
            if let Some(language) = matches.value_of("lang") {
                lang = language;
            }
        }
    }

    match lang {
        "go" => {
            println!("{}", go::render::render_html(file_path));
        }
        "rust" => {
            println!("{}", go::render::render_html(file_path));
        }
        _ => {
            println!("Language {} not supported", lang);
        }
    }
}
