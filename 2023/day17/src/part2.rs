use crate::ultra;

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

pub fn solve(input: &[String]) {
    let mut grid: Grid = vec![];
    for line in input {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        );
    }

    let start: Position = (0, 0);
    let end: Position = (grid[0].len()-1, grid.len()-1);
    let cost = ultra::find_path(&grid, start, end);
    let end: Vec<_> = cost.iter()
        .filter(|(k, _)| k.0 == end.0 && k.1 == end.1)
        .collect();
    println!("{:?}", end);
}