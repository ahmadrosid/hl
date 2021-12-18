use clap::{arg, App};
use std::fs;

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
        let s = parse_file_path(filepath);
        println!("{}", s);
    }
}

fn parse_file_path(path: &str) -> String {
    let source_code = fs::read(path).expect(&format!("Filed reading file {}", path));
    if Some(&source_code) == None {
        return "".to_string();
    }

    let mut html = String::new();
    // html.push_str("<!DOCTYPE html>\n");
    // html.push_str("<html lang=\"en\">\n");
    // html.push_str("<head>\n");
    // html.push_str("  <meta charset=\"utf-8\">\n");
    // html.push_str("  <link rel=\"stylesheet\" href=\"style.css\">\n");
    // html.push_str("</head>\n");
    // html.push_str("<body>\n");

    html.push_str("<table class=\"highlight-table\">\n");
    html.push_str("<tbody>\n");
    
    let mut i = 0;
    let char_length = source_code.len();
    let mut total = 1;
    for n in source_code {
        if i == 0 {
            html.push_str("<tr data-line=\"1\">");
            html.push_str("\n<td><div class=\"lineno\">1</div></td>\n");
            html.push_str("<td><span>");
            i = i + 1;
        }

        let c = n as char;
        if c == '\n' {
            i = i + 1;
            if total != char_length {
                html.push_str("</span>");
                html.push_str("</td>\n</tr>");
                html.push_str(&format!("\n<tr data-line=\"{}\">", i));
                html.push_str(&format!("\n<td><div class=\"lineno\">{}</div></td>\n", i));
                html.push_str("<td><span>\n");
            } else {
                html.push_str("</span>");
                html.push_str("</td>\n</tr>");
            }
        }

        html.push_str(&format!("{}", c));
        total = total + 1;
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>\n");

    // html.push_str("</body>\n");
    // html.push_str("</html>\n");
    html
}
