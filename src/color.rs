pub fn green(s: &str) -> String {
    return format!("\x1b[0;32m{}\x1b[m", s);
}

pub fn bold_green(s: &str) -> String {
    return format!("\x1b[1;32m{}\x1b[m", s);
}

pub fn cyan(s: &str) -> String {
    return format!("\x1b[0;96m{}\x1b[m", s);
}