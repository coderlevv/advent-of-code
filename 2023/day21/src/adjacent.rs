type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type PositionInf = (usize, usize, i32, i32);

pub fn adjacent(
    grid: &Grid,
    current: &Position
) -> Vec<Position> {
    let mut adj: Vec<Position> = Vec::new();
    let x = current.0;
    let y = current.1;
    if x >= 1 && grid[y][x-1] == '.' {
        adj.push((x-1, y));
    }
    if x < grid[0].len()-1 && grid[y][x+1] == '.' {
        adj.push((x+1, y));
    }
    if y >= 1 && grid[y-1][x] == '.' {
        adj.push((x, y-1));
    }
    if y < grid.len()-1 && grid[y+1][x] == '.' {
        adj.push((x, y+1));
    }
    adj
}

pub fn adjacent_inf(
    grid: &Grid,
    current: &PositionInf
) -> Vec<PositionInf> {
    let mut adj: Vec<PositionInf> = Vec::new();
    let x = current.0;
    let y = current.1;
    let zx: i32 = current.2;
    let zy: i32 = current.3;

    if x >= 1 {
        if grid[y][x-1] == '.' {
            adj.push((x-1, y, zx, zy));
        }
    } else {
        adj.push((grid[0].len()-1, y, zx-1, zy));
    }

    if x < grid[0].len()-1 {
        if grid[y][x+1] == '.' {
            adj.push((x+1, y, zx, zy));
        }
    } else {
        adj.push((0, y, zx+1, zy));
    }

    if y >= 1 {
        if grid[y-1][x] == '.' {
            adj.push((x, y-1, zx, zy));
        }
    } else {
        adj.push((x, grid.len()-1, zx, zy-1));
    }

    if y < grid.len()-1 {
        if grid[y+1][x] == '.' {
            adj.push((x, y+1, zx, zy));
        }
    } else {
        adj.push((x, 0, zx, zy+1));
    }
    adj
}
