use clap::{arg, App, AppSettings};
pub mod color;
pub mod module;
pub mod parser;
pub mod render;
pub mod string;
pub mod token;

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Generate lexer, this is for development only.")
        .args(&[
            arg!(-i --input <FILE> "File path to generate lexer"),
            arg!(-o --output <DIR> "Output folder for lexer"),
        ])
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let lexer_path = matches.value_of("input").expect("required");
    let output_path = matches.value_of("output").expect("required");
    println!("{}", parser::parse(lexer_path, output_path));
}
