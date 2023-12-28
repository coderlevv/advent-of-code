pub fn solve(input: &[String]) {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect());
    }
    
    tilt_north(&mut grid);
    //display(&grid);
    println!("{}", weight(&grid));
}

fn tilt_north(grid: &mut [Vec<char>]) {
    let mut change = true;
    while change {
        change = false;
        for y in 1..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'O' && grid[y-1][x] == '.' {
                    grid[y][x] = '.';
                    grid[y-1][x] = 'O';
                    change = true;
                }
            }
        }
    }
}

fn weight(grid: &[Vec<char>]) -> u32 {
    let mut load: u32 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (_, c) in row.iter().enumerate() {
            if *c == 'O' {
                load += (grid.len() - y) as u32;
            }
        }
    }
    load
}

pub fn display(grid: &[Vec<char>]) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}