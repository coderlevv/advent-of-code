use std::fs::File;
use std::io::{BufRead, BufReader};

mod solve;
mod stone;

fn main() {
    let mut input = Vec::new();
    let file_name = "input";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => input.push(line),
            Err(_) => panic!("IO failed!")
        }
    }
    
    solve::solve(&input);
}
