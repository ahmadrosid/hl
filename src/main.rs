use clap::{App, arg};
mod lexer;

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax highlighting.")
        .arg(arg!([filepath] "File path to parse."))
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values")),
        )
        .get_matches();

    if let Some(filepath) = matches.value_of("filepath") {
        let s = lexer::render::parse_file_path(filepath);
        println!("{}", s);
    }
}
