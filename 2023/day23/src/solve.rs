use std::collections::HashSet;
use crate::path;
use std::thread;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

pub fn solve(input: &[String]) {
    let mut grid: Grid = vec![];
    for line in input {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let width = grid[0].len();
    let height = grid.len();

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    // scan map horizontally for transporter pods
    for y in 0..height {
        for x in 0..width-1 {
            let c = grid[y][x];
            if y == 0 && c == '.' {
                start = (x, y);
            }
            if y == height - 1 && c == '.' {
                end = (x, y);
            }
        }
    }

    let mut visited: HashSet<Position> = HashSet::new();
    let mut max_steps: u32 = 0;
    path::search(0, start, end, &grid, &mut visited, &mut max_steps, true);
    println!("Solution part 1: {:?}", max_steps);

    // create a thread with increased stack size
    let child = thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(move || { 
        let mut max_steps = 0;
        path::search(0, start, end, &grid, &mut visited, &mut max_steps, false);
        println!("Solution part 2: {:?}", max_steps);
     }).unwrap();
     child.join().unwrap();
}