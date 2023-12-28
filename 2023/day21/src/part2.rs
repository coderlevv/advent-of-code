use crate::adjacent;
use crate::extrapolate::extrapolate;
use std::collections::HashSet;
type Grid = Vec<Vec<char>>;
type PositionInf = (usize, usize, i32, i32);

pub fn solve(input: &[String]) {
    let mut grid: Grid = vec![];
    let mut start: PositionInf = (0, 0, 0, 0);
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' { start = (x, y, 0, 0); }
            row.push(c);
        }
        grid.push(row);
    }
    grid[start.1][start.0] = '.';
    let mut reached: HashSet<PositionInf> = HashSet::new();
    let mut tmp: HashSet<PositionInf> = HashSet::new();
    reached.insert(start);
    let mut step: u64 = 0;
    while step < 1 {
        step += 1;
        for pos in &reached {
            let adj = adjacent::adjacent_inf(&grid, pos);
            tmp.extend(adj.iter());
        }
        reached = tmp.clone();
        tmp.clear();
    }

    // thanks to eric burden for the idea
    // https://www.ericburden.work/blog/2023/12/21/advent-of-code-day-21/
    let mut seq: Vec<i64> = vec![33270, 92194, 180510, 298218];
    let periods: u64 = (26501365 - 65) / 131 - 4;
    for _ in 0..periods {
        seq.push(extrapolate(&seq[seq.len()-3..seq.len()]));
    }
    println!("{:?}", seq[seq.len()-1]);
}