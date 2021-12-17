use clap::{arg, App};
use std::fs;
// use std::path::Path;

fn main() {
    let matches = App::new("hl")
        .version("1.0")
        .author("Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Syntax hightlighting.")
        .arg(
            arg!([filepath] "File path to parse.")
        )
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values")),
        )
        .get_matches();

    if let Some(filepath) = matches.value_of("filepath") {
        println!("Parsing filepath: {}\n", filepath);
        parse_file_path(filepath)
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}

fn parse_file_path(path: &str) {
    let source_code =
        fs::read(path).expect(&("Filed reading file ".to_owned() + path));
    if Some(&source_code) == None {
        return
    }

    let s = match std::str::from_utf8(&source_code) {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse source code to string {}", e),
    };
    
    println!("{}", s)
}