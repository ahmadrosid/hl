use clap::{arg, App};

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
        println!("Value for filepath: {}", filepath);
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
