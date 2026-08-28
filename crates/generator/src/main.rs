use generator::parser;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut input = None;
    let mut output = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input" => input = args.next(),
            "-o" | "--output" => output = args.next(),
            _ => {
                eprintln!("Usage: hl-gen -i <FILE> -o <DIR>");
                std::process::exit(1);
            }
        }
    }
    let (Some(input), Some(output)) = (input, output) else {
        eprintln!("Usage: hl-gen -i <FILE> -o <DIR>");
        std::process::exit(1);
    };
    println!("{}", parser::parse(&input, &output));
}
