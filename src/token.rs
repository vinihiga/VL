pub fn tokenize(line_content: &str) -> Vec<String> {
    return line_content.split(" ").map(|s| s.to_string()).collect();
}