use clap::{arg, App};
mod render;

fn main() {
    let matches = App::new("hl")
        .version("1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax hightlighting.")
        .arg(arg!([filepath] "File path to parse."))
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values")),
        )
        .get_matches();

    if let Some(filepath) = matches.value_of("filepath") {
        let s = render::parse_file_path(filepath);
        println!("{}", s);
    }
}

