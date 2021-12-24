// The comment section!
fn main() {
    let matches = App::new("hl")
        .version("0.1.0");
    let mut ada = 5;
    shadow!(shadowing);
    if true {
        String::new();
        let a : Vec<char> = vec!['0'];
    }
}

/**
* This is double star comment!
*/
fn process(a: &str, b: char) {
    println!(a, b);
}
