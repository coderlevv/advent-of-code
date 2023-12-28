use std::collections::{BinaryHeap, HashMap};

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

pub fn adjacent_pipe(
    grid: &Grid,
    current: &Position
) -> Vec<Position> {
    let mut adj: Vec<Position> = Vec::new();
    let x = current.0;
    let y = current.1;
    match grid[y][x] {
        '|' => {
            // N
            if y > 0 && (grid[y-1][x] == '7' || grid[y-1][x] == 'F' || grid[y-1][x] == '|') {
                adj.push((x, y-1));
            }
            // S
            if y < grid.len()-1 && (grid[y+1][x] == 'L' || grid[y+1][x] == 'J' || grid[y+1][x] == '|') {
                adj.push((x, y+1));
            }
        },
        '-' => {
            // W
            if x > 0 && (grid[y][x-1] == 'L' || grid[y][x-1] == 'F' || grid[y][x-1] == '-') {
                adj.push((x-1, y));
            }
            // E
            if x < grid[0].len()-1 && (grid[y][x+1] == 'J' || grid[y][x+1] == '7' || grid[y][x+1] == '-') {
                adj.push((x+1, y));
            }
        },
        'L' => {
            // N
            if y > 0 && (grid[y-1][x] == '7' || grid[y-1][x] == 'F' || grid[y-1][x] == '|') {
                adj.push((x, y-1));
            }
            // E
            if x < grid[0].len()-1 && (grid[y][x+1] == 'J' || grid[y][x+1] == '7' || grid[y][x+1] == '-') {
                adj.push((x+1, y));
            }
        },
        'J' => {
            // N
            if y > 0 && (grid[y-1][x] == '7' || grid[y-1][x] == 'F' || grid[y-1][x] == '|') {
                adj.push((x, y-1));
            }
            // W
            if x > 0 && (grid[y][x-1] == 'L' || grid[y][x-1] == 'F' || grid[y][x-1] == '-') {
                adj.push((x-1, y));
            }
        },
        '7' => {
            // S
            if y < grid.len()-1 && (grid[y+1][x] == 'L' || grid[y+1][x] == 'J' || grid[y+1][x] == '|') {
                adj.push((x, y+1));
            }
            // W
            if x > 0 && (grid[y][x-1] == 'L' || grid[y][x-1] == 'F' || grid[y][x-1] == '-') {
                adj.push((x-1, y));
            }
        },
        'F' => {
            // S
            if y < grid.len()-1 && (grid[y+1][x] == 'L' || grid[y+1][x] == 'J' || grid[y+1][x] == '|') {
                adj.push((x, y+1));
            }
             // E
             if x < grid[0].len()-1 && (grid[y][x+1] == 'J' || grid[y][x+1] == '7' || grid[y][x+1] == '-') {
                adj.push((x+1, y));
            }
        },
        'S' => {
            // N
            if y > 0 && (grid[y-1][x] == '7' || grid[y-1][x] == 'F' || grid[y-1][x] == '|') {
                adj.push((x, y-1));
            }
            // S
            if y < grid.len()-1 && (grid[y+1][x] == 'L' || grid[y+1][x] == 'J' || grid[y+1][x] == '|') {
                adj.push((x, y+1));
            }
            // W
            if x > 0 && (grid[y][x-1] == 'L' || grid[y][x-1] == 'F' || grid[y][x-1] == '-') {
                adj.push((x-1, y));
            }
            // E
            if x < grid[0].len()-1 && (grid[y][x+1] == 'J' || grid[y][x+1] == '7' || grid[y][x+1] == '-') {
                adj.push((x+1, y));
            }
        }
        _ => panic!("Dead end!")
    }
    adj
}

pub fn find_path(
    grid: &Grid,
    start: &Position,
) -> HashMap<Position, u32> {
    let mut front: BinaryHeap<Position> = BinaryHeap::new();
    front.push(*start);
    let mut from: HashMap<Position, Option<Position>> = HashMap::new();
    let mut cost: HashMap<Position, u32> = HashMap::new();
    cost.insert(*start, 0);
    from.insert(*start, None);
    while !front.is_empty() {
        if let Some(current) = front.pop() {
            for next in adjacent_pipe(grid, &current) {
                let current_cost = *cost.get(&current).unwrap();
                let new_cost = current_cost + 1;
                if !cost.contains_key(&next) || new_cost < *cost.get(&next).unwrap() {
                    cost.insert(next, new_cost);
                    front.push(next);
                    from.insert(next, Some(current));
                }
            }
        }
    }
    cost
}

pub fn adjacent(
    grid: &Grid,
    current: &Position
) -> Vec<Position> {
    let mut adj: Vec<Position> = Vec::new();
    let x = current.0;
    let y = current.1;
    if y > 0 && grid[y-1][x] == '.' {
        adj.push((x, y-1));
    }
    if y > 0 && x < grid[0].len()-1 && grid[y-1][x+1] == '.' {
        adj.push((x+1, y-1));
    }
    if x < grid[0].len()-1 && grid[y][x+1] == '.' {
        adj.push((x+1, y));
    }
    if y < grid.len()-1 && x < grid[0].len()-1 && grid[y+1][x+1] == '.' {
        adj.push((x+1, y+1));
    }
    if y < grid.len()-1 && grid[y+1][x] == '.' {
        adj.push((x, y+1));
    }
    if y < grid.len()-1 && x > 0 && grid[y+1][x-1] == '.' {
        adj.push((x-1, y+1));
    }
    if x > 0 && grid[y][x-1] == '.' {
        adj.push((x-1, y));
    }
    if y > 0 && x > 0 && grid[y-1][x-1] == '.' {
        adj.push((x-1, y-1));
    }
    adj
}

fn fill_from_pos(
    grid: &mut Grid,
    start: &Position
) {
    if grid[start.1][start.0] != '.' {
        return;
    }
    let mut front: BinaryHeap<Position> = BinaryHeap::new();
    front.push(*start);
    let mut from: HashMap<Position, Option<Position>> = HashMap::new();
    from.insert(*start, None);
    while !front.is_empty() {
        if let Some(current) = front.pop() {
            grid[current.1][current.0] = 'O';
            for next in adjacent(grid, &current) {
                from.entry(next).or_insert_with(|| {
                    front.push(next);
                    Some(current)
                });
            }
        }
    }
}

pub fn fill(grid: &mut Grid) {
    for x in 0..grid[0].len()-1 {
        fill_from_pos(grid, &(x, 0));
        fill_from_pos(grid, &(x, grid.len()-1));
    }
    for y in 0..grid.len()-1 {
        fill_from_pos(grid, &(0, y));
        fill_from_pos(grid, &(grid[0].len()-1, y));
    }
}

// pub fn display(grid: &Grid, /* pipe: &[Position] */) {
//     for (y, row) in grid.iter().enumerate() {
//         for (x, c) in row.iter().enumerate() {
//             // if pipe.contains(&(x, y)) { print!("{}", c); }
//             // else { print!("."); }
//             print!("{}", c);
//         }
//         println!();
//     }
// }