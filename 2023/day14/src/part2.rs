use csv::Writer;

pub fn solve(input: &[String]) {
    let mut init: Vec<Vec<char>> = vec![];
    for line in input {
        init.push(line.chars().collect());
    }
    
    let mut grid = init.clone();
    let mut seq: Vec<u32> = vec![];
    for _ in 0..1000 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        seq.push(weight(&grid));
    }
    let mut writer= Writer::from_path("sequence.csv").unwrap();
    writer.write_record(seq.iter().map(|s| s.to_string())).unwrap();
}

fn is_same(a: &[Vec<char>], b: &[Vec<char>]) -> bool {
    let mut is_init = true;
    for y in 0..a.len() {
        for x in 0..a[0].len() {
            if a[y][x] != b[y][x] {
                is_init = false;
                break;
            }
        }
    }
    is_init
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

fn tilt_south(grid: &mut [Vec<char>]) {
    let mut change = true;
    while change {
        change = false;
        for y in (0..grid.len()-1).rev() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'O' && grid[y+1][x] == '.' {
                    grid[y][x] = '.';
                    grid[y+1][x] = 'O';
                    change = true;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut [Vec<char>]) {
    let mut change = true;
    while change {
        change = false;
        for y in 0..grid.len() {
            for x in 1..grid[0].len() {
                if grid[y][x] == 'O' && grid[y][x-1] == '.' {
                    grid[y][x] = '.';
                    grid[y][x-1] = 'O';
                    change = true;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut [Vec<char>]) {
    let mut change = true;
    while change {
        change = false;
        for y in 0..grid.len() {
            for x in (0..grid[0].len()-1).rev() {
                if grid[y][x] == 'O' && grid[y][x+1] == '.' {
                    grid[y][x] = '.';
                    grid[y][x+1] = 'O';
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