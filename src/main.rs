use std::fs::File;
use std::io::{BufRead, BufReader};
mod token;
mod lexical;
mod processor;

fn main() {
    read_file();
}

fn read_file() {
    let file = File::open("./samples/test.vl.vl").unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = token::tokenize(&line[..]);
        // TODO: AST data structure for better handling.
        lexical::fix_expressions(&mut tokens);
        processor::calculate_sentence(&tokens.join(" ").to_string()[..]);
    }
}