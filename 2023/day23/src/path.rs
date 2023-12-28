type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

use std::collections::HashSet;

const ALLOWED: [char; 5] = ['.', '<', '>', 'v', '^'];

fn adjacent(
    grid: &Grid,
    current: &Position,
    slippery: bool
) -> Vec<Position> {
    let mut adj: Vec<Position> = Vec::new();
    let x = current.0;
    let y = current.1;
    let c = grid[y][x];
    if slippery {
        match c {
            '.' => {
                if x > 0 && ALLOWED.contains(&grid[y][x-1]) {
                    adj.push((x-1, y));
                }
                if x < grid[0].len()-1 && ALLOWED.contains(&grid[y][x+1]) {
                    adj.push((x+1, y));
                }
                if y > 0 && ALLOWED.contains(&grid[y-1][x]) {
                    adj.push((x, y-1));
                }
                if y < grid.len()-1 && ALLOWED.contains(&grid[y+1][x]) {
                    adj.push((x, y+1));
                }
            },
            '>' => adj.push((x+1, y)),
            '<' => adj.push((x-1, y)),
            'v' => adj.push((x, y+1)),
            '^' => adj.push((x, y-1)),
            _ => panic!("Unnown tile!")
        }
    } else {
        if x > 0 && ALLOWED.contains(&grid[y][x-1]) {
            adj.push((x-1, y));
        }
        if x < grid[0].len()-1 && ALLOWED.contains(&grid[y][x+1]) {
            adj.push((x+1, y));
        }
        if y > 0 && ALLOWED.contains(&grid[y-1][x]) {
            adj.push((x, y-1));
        }
        if y < grid.len()-1 && ALLOWED.contains(&grid[y+1][x]) {
            adj.push((x, y+1));
        }
    }
    adj
}

pub fn search(
    step: u32,
    pos: Position,
    end: Position,
    grid: &Grid,
    visited: &mut HashSet<Position>,
    max_steps: &mut u32,
    slippery: bool
) {
    visited.insert(pos);
    if pos == end {
        if step > *max_steps {
            *max_steps = step;
        }
    } else {
        let next_pos = adjacent(grid, &pos, slippery);
        for next in next_pos {
            if !visited.contains(&next) {
                search(step+1, next, end, grid, visited, max_steps, slippery);
                visited.remove(&next);
            }
        }
    }
}
