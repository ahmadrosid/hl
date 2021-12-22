use clap::{arg, App, AppSettings};
mod generator;
// mod lexer;
mod lexers;
use crate::lexers::rust;

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax highlighting.")
        .arg(arg!([FILE_PATH] "File path to parse."))
        .subcommand(
            App::new("generate")
                .short_flag('g')
                .long_flag("generate")
                .about("Generate lexer.")
                .arg(arg!([LEXER_PATH] "Lexer path"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let lexer_path = sub_matches.value_of("LEXER_PATH").expect("required");
            let s = generator::parse(lexer_path);
            println!("Generating {}\n{}", lexer_path, s);
        }
        _ => {
            if let Some(filepath) = matches.value_of("FILE_PATH") {
                let s = rust::render::parse_file_path(filepath);
                println!("{}", s);
            }
        }
    }
}
