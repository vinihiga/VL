use std::fs::File;
use std::io::{BufRead, BufReader};
mod token;
mod semantic;

fn main() {
    read_file();
}

fn read_file() {
    let file = File::open("./samples/signals.vl").unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = token::tokenize(&line[..]);
        semantic::fix_expressions(&mut tokens);
        println!("{}", tokens.join(" "));
    }
}