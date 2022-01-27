use clap::{arg, App, AppSettings};
mod generator;
use hl::render_html;

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
            std::process::exit(0x000);
        }
        _ => {
            if let Some(file) = matches.value_of("FILE_PATH") {
                let source = std::fs::read_to_string(file).expect(&format!("Filed reading file {}", file));
                input = source.chars().collect::<Vec<_>>();
            }
            if let Some(language) = matches.value_of("lang") {
                lang = language;
            }
        }
    }
    print!("{}", render_html(input, lang));
}
