use clap::{arg, App, AppSettings};
mod generator;
mod lexers;
use crate::lexers::{c, cpp, css, go, java, javascript, php, rust, typescript};
use std::fs::read;
use std::io::Write;

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
        "c" => {
            let content = c::render::render_html(input);
            print!("{}", content);
        }
        "go" => {
            let content = go::render::render_html(input);
            print!("{}", content);
        }
        "rust" => {
            print!("{}", rust::render::render_html(input));
        }
        "css" => {
            print!("{}", css::render::render_html(input));
        }
        "cpp" => {
            print!("{}", cpp::render::render_html(input));
        }
        "java" => {
            print!("{}", java::render::render_html(input));
        }
        "js" => {
            print!("{}", javascript::render::render_html(input));
        }
        "php" => {
            print!("{}", php::render::render_html(input));
        }
        "ts" => {
            print!("{}", typescript::render::render_html(input));
        }
        _ => {
            println!("Language {} not supported", lang);
        }
    }
}

#[allow(dead_code)]
fn write_file(content: &String, file_name: &str) {
    let cwd = std::env::current_dir().unwrap();
    let mut file =
        std::fs::File::create(format!("{}/{}", cwd.to_str().unwrap(), file_name)).unwrap();
    write!(&mut file, "{}", content).unwrap();
}
