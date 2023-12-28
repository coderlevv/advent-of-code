use crate::path;
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
    let cost = path::find_path(&grid, &start);
    // solution part 1
    println!("{:?}", cost.values().max());

    let pipe = Vec::from_iter(cost.keys().copied());
    path::fill(&mut grid);
    let mut xgrid = expand(&grid, &pipe);
    path::fill(&mut xgrid);
    let mut count: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' { continue; }
            if !pipe.contains(&(x, y)) && inside(&xgrid, &(x, y)) {
                count += 1;
            }
        }
    }
    // solution part 2
    println!("{:?}", count);
}

fn inside(
    grid: &Grid,
    pos: &Position
) -> bool {
    let ix = pos.0 * 3 + 1;
    let iy = pos.1 * 3 + 1;
    grid[iy][ix] != 'O'
}

fn expand(grid: &Grid, pipe: &[Position]) -> Vec<Vec<char>> {
    let mut xgrid: Vec<Vec<char>> = vec![];
    //let ydim = grid.len();
    //let xdim = grid[0].len();
    for (y, row) in grid.iter().enumerate() {
        let mut xrow1: Vec<char> = vec![];
        let mut xrow2: Vec<char> = vec![];
        let mut xrow3: Vec<char> = vec![];
        for (x, c) in row.iter().enumerate() {
                if pipe.contains(&(x, y)) {
                    match *c {
                        '|' => { 
                            xrow1.extend(".|.".chars());
                            xrow2.extend(".|.".chars());
                            xrow3.extend(".|.".chars());
                        },
                        '-' => {
                            xrow1.extend("...".chars());
                            xrow2.extend("---".chars());
                            xrow3.extend("...".chars());
                        },
                        'L' => {
                            xrow1.extend(".|.".chars());
                            xrow2.extend(".L-".chars());
                            xrow3.extend("...".chars());
                        },
                        'J' => { 
                            xrow1.extend(".|.".chars());
                            xrow2.extend("-J.".chars());
                            xrow3.extend("...".chars());
                        },
                        '7' => { 
                            xrow1.extend("...".chars());
                            xrow2.extend("-7.".chars());
                            xrow3.extend(".|.".chars());
                        },
                        'F' => { 
                            xrow1.extend("...".chars());
                            xrow2.extend(".F-".chars());
                            xrow3.extend(".|.".chars());
                        },
                        'S' => { 
                            xrow1.extend("SSS".chars());
                            xrow2.extend("SSS".chars());
                            xrow3.extend("SSS".chars());
                        },
                        // '.' => {
                        //     xrow1.extend("...".chars());
                        //     xrow2.extend("...".chars());
                        //     xrow3.extend("...".chars());
                        // }
                        _ => panic!("Unknown pipe part!")
                    }
                } else {
                    xrow1.extend("...".chars());
                    xrow2.extend("...".chars());
                    xrow3.extend("...".chars());
                }
        }
        xgrid.push(xrow1);
        xgrid.push(xrow2);
        xgrid.push(xrow3);
        //path::display(&xgrid);
        //println!();
    }
    xgrid
}
