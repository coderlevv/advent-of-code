pub fn solve(input: &[String]) {
    let mut cols: Vec<usize> = vec![];
    let mut rows: Vec<usize> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        if line.is_empty() {
            if let Some(col) = find_vertical(&grid) {
                cols.push(col+1);
            }
            if let Some(row) = find_horizontal(&grid) {
                rows.push(row+1);
            }
            grid.clear();
        }
        else {
            grid.push(
                line.chars().collect()
            );
        }
    }
    if !grid.is_empty() {
        if let Some(col) = find_vertical(&grid) {
            cols.push(col+1);
        }
        if let Some(row) = find_horizontal(&grid) {
            rows.push(row+1);
        }
    }
    println!("{:?}", cols.iter().sum::<usize>() + rows.iter().sum::<usize>() * 100);
}

pub fn find_vertical(grid: &[Vec<char>]) -> Option<usize> {
    let mut reflect: usize = 0;
    let x_len = grid[0].len();
    let mut found = false;
    'outer: for r in 0..grid[0].len()-1 {
        found = true;
        reflect = r;
        let mut k: usize = 0;
        'inner: while k <= r && r+1+k < x_len { 
            for y in 0..grid.len() {
                if grid[y][r-k] != grid[y][r+1+k] {
                    found = false;
                    break 'inner;
                }
            }
            k += 1;
        }
        if found { break 'outer } 
    }
    if !found { return None }
    Some(reflect)
}

pub fn find_horizontal(grid: &[Vec<char>]) -> Option<usize> {
    let mut reflect: usize = 0;
    let y_len = grid.len();
    let mut found = false;
    'outer: for r in 0..grid.len()-1 {
        found = true;
        reflect = r;
        let mut k: usize = 0;
        'inner: while k <= r && r+1+k < y_len { 
            for x in 0..grid[0].len() {
                if grid[r-k][x] != grid[r+1+k][x] {
                    found = false;
                    break 'inner;
                }
            }
            k += 1;
        }
        if found { break 'outer } 
    }
    if !found { return None }
    Some(reflect)
}