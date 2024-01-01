use crate::path;

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
    //println!("{:?}", grid);
    let start: Position = (0, 0);
    let end: Position = (grid[0].len()-1, grid.len()-1);
    let cost = path::find_path(&grid, start, end);
    let mut res: Vec<_> = cost.iter()
        .filter(|(k, _)| k.0 == end.0 && k.1 == end.1)
        .collect();
    res.sort_by(|a, b| a.1.cmp(b.1));
    println!("{:?}", res[0].1);
}