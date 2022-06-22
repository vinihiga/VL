use std::fs;
mod tokenizer;

fn main() {
    read_file("./samples/addition.vl")
}

fn read_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    tokenizer::tokenize(&contents);
}