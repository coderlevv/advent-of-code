type Grid = Vec<Vec<u32>>;

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    N, W, S, E
}

#[derive(Debug)]
pub struct Node {
    pos: (usize, usize),
    dir: Option<Direction>,
    cost: u32
}

impl Node {
    pub fn new(pos: (usize, usize), dir: Option<Direction>, cost: u32) -> Self {
        Node {
            pos,
            dir,
            cost
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn adjacent(
    grid: &Grid,
    current: &Node,
) -> Vec<(usize, usize, Direction, u32)> {
    let mut adj: Vec<(usize, usize, Direction, u32)> = Vec::new();
    let x = current.pos.0;
    let y = current.pos.1;
    let mut cost: u32;
    if let Some(dir) = current.dir {
        match dir {
            Direction::N => {
                cost = 0;
                for s in 1..=3 {
                    if x >= s {
                        cost += grid[y][x-s];
                        adj.push((x-s, y, Direction::W, cost));
                    }
                }
                cost = 0;
                for s in 1..=3 {
                    if x+s < grid[0].len() {
                        cost += grid[y][x+s];
                        adj.push((x+s, y, Direction::E, cost));
                    }
                }
            },
            Direction::W => {
                cost = 0;
                for s in 1..=3 {
                    if y >= s {
                        cost += grid[y-s][x];
                        adj.push((x, y-s, Direction::N, cost));
                    }
                }
                cost = 0;
                for s in 1..=3 {
                    if y+s < grid.len() {
                        cost += grid[y+s][x];
                        adj.push((x, y+s, Direction::S, cost));
                    }
                }
            },
            Direction::S => {
                cost = 0;
                for s in 1..=3 {
                    if x >= s {
                        cost += grid[y][x-s];
                        adj.push((x-s, y, Direction::W, cost));
                    }
                }
                cost = 0;
                for s in 1..=3 {
                    if x+s < grid[0].len() {
                        cost += grid[y][x+s];
                        adj.push((x+s, y, Direction::E, cost));
                    }
                }
            },
            Direction::E => {
                cost = 0;
                for s in 1..=3 {
                    if y >= s {
                        cost += grid[y-s][x];
                        adj.push((x, y-s, Direction::N, cost));
                    }
                }
                cost = 0;
                for s in 1..=3 {
                    if y+s < grid.len() {
                        cost += grid[y+s][x];
                        adj.push((x, y+s, Direction::S, cost));
                    }
                }
            }
        }
    } else {
        cost = 0;
        for s in 1..=3 {
            if y+s < grid.len() {
                cost += grid[y+s][x];
                adj.push((x, y+s, Direction::S, cost));
            }
        }
        cost = 0;
        for s in 1..=3 {
            if x+s < grid[0].len() {
                cost += grid[y][x+s];
                adj.push((x+s, y, Direction::E, cost));
            }
        }
    }
    adj
}

pub fn find_path(
    grid: &Grid,
    start: (usize, usize),
    end: (usize, usize)
    ) -> /* Vec<(usize, usize)> */ HashMap<(usize, usize, u32), u32> {
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    frontier.push(Node::new(start, None, 0));
    let mut came_from: HashMap<(usize, usize, u32), Option<(usize, usize)>> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize, u32), u32> = HashMap::new();
    cost_so_far.insert((start.0, start.1, Direction::E as u32), 0);
    came_from.insert((start.0, start.1, Direction::E as u32), None);
    while !frontier.is_empty() {
        if let Some(current) = frontier.pop() {
            if current.pos == end {
                break;
            }
            let next_vec = adjacent(grid, &current);
            for next in next_vec {
                let new_cost = current.cost + next.3;
                if !cost_so_far.contains_key(&(next.0, next.1, next.2 as u32)) || new_cost < *cost_so_far.get(&(next.0, next.1, next.2 as u32)).unwrap() {
                    cost_so_far.insert((next.0, next.1, next.2 as u32), new_cost);
                    frontier.push(Node::new((next.0, next.1), Some(next.2), new_cost));
                    came_from.insert((next.0, next.1, next.2 as u32), Some(current.pos));
                }
            }
        }
    }
    // println!("{:?}", cost_so_far.get(&end).copied());
    // let mut path: Vec<(usize, usize)> = vec![end];
    // if found {
    //     let mut stop = false;
    //     let mut node = end;
    //     while !stop {
    //         if let Some(pos) = came_from.get(&node).unwrap() {
    //             path.push(*pos);
    //             node = *pos;
    //         } else {
    //             stop = true;
    //         }
    //     }
    // }
    // path
    cost_so_far
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