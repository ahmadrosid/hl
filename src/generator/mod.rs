pub mod token;

pub fn parse(filepath: &str) -> String {
    let file = std::fs::read(filepath).expect(&format!("Can not read file path: {}", filepath));
    let content = file.iter().map(|c| *c as char).collect::<Vec<_>>();
    let mut result = String::new();
    result.push_str(&String::from_iter(content));
    result
}
