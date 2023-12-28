use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

fn main() {
    let mut input: Vec<Vec<char>> = Vec::new();
    let file_name = "input";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => input.push(line.chars().collect()),
            Err(_) => panic!("IO failed!")
        }
    }
    
    part1::solve(&input);
    part2::solve(&input);
}