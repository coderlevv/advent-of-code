use crate::adjacent;
use std::collections::HashSet;
type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

pub fn solve(input: &[String]) {
    let mut grid: Grid = vec![];
    let mut start: Position = (0, 0);
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' { start = (x, y); }
            row.push(c);
        }
        grid.push(row);
    }
    grid[start.1][start.0] = '.';
    let mut reached: HashSet<Position> = HashSet::new();
    let mut tmp: HashSet<Position> = HashSet::new();
    reached.insert(start);
    let mut step: u32 = 0;
    while step < 64 {
        step += 1;
        for pos in reached {
            let adj = adjacent::adjacent(&grid, &pos);
            tmp.extend(adj.iter());
        }
        reached = tmp.clone();
        tmp.clear();
    }
    println!("{:?}", reached.len());
}