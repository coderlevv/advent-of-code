use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;
mod path;
mod ultra;

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
    
    part1::solve(&input);
    part2::solve(&input);
}
