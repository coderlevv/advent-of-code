use std::collections::HashSet;

pub fn solve(input: &[String]) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut empty_row: Vec<usize> = vec![];
    let mut cols: HashSet<usize> = HashSet::new();
    for (y, line) in input.iter().enumerate() {
        let mut row_flag = false;
        let mut row: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == '#' {
                 row_flag = true;
                 cols.insert(x);
            }
        }
        if !row_flag { empty_row.push(y); }
        grid.push(row);
    }
    let empty_cols: Vec<usize> = (0..input[0].len())
        .filter(|c| !cols.contains(c))
        .collect();
     // solution part 1
    let galaxies: Vec<(usize, usize)> = expand(&grid, &empty_row, &empty_cols, 1);
    let dist = distance(&galaxies);
    println!("{:?}", dist.iter().sum::<i64>());
    // solution part 2
    let galaxies: Vec<(usize, usize)> = expand(&grid, &empty_row, &empty_cols, 999999);
    let dist = distance(&galaxies);
    println!("{:?}", dist.iter().sum::<i64>());
}

fn expand(
    grid: &[Vec<char>],
    empty_row: &[usize],
    empty_col: &[usize],
    fold: usize
) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut ky = 0;
    for (y, row) in grid.iter().enumerate() {
        if empty_row.contains(&y) {
            ky += fold;
        }
        let mut kx = 0;
        for (x, c) in row.iter().enumerate() {
            if empty_col.contains(&x) {
                kx += fold;
            }
            if *c == '#' {
                galaxies.push((x + kx, y + ky));
            }
        }
    }
    galaxies
}

fn distance(galaxies: &[(usize, usize)]) -> Vec<i64> {
    let mut dist: Vec<i64> = vec![];
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            dist.push(
                (g2.0 as i64 - g1.0 as i64).abs() + (g2.1 as i64 - g1.1 as i64).abs() 
            );
        }
    }
    dist
}
