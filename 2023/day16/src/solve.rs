use crate::beam::{Beam, Status, Direction};
use std::collections::HashSet;

const STOPPING: usize = 10;

pub fn solve(input: &[String]) {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect());
    }

    // solution part 1
    let n = run_light(&grid, Beam::new((0, 0), Direction::Right));
    println!("{:?}", n);
     
    // solution part 2
    // takes quite long to complete!
    let mut n_energized: Vec<usize> = vec![];
    for x in 0..grid[0].len() {
        println!("x: {} of {}", x, grid[0].len());
        let start = (x, 0);
        let n = run_light(&grid, Beam::new(start, Direction::Down));
        n_energized.push(n);
        let start = (x, grid[0].len()-1);
        let n = run_light(&grid, Beam::new(start, Direction::Up));
        n_energized.push(n);
    }
    for y in 0..grid.len() {
        println!("y: {} of {}", y, grid.len());
        let start = (0, y);
        let n = run_light(&grid, Beam::new(start, Direction::Right));
        n_energized.push(n);
        let start = (grid[0].len()-1, y);
        let n = run_light(&grid, Beam::new(start, Direction::Left));
        n_energized.push(n);
    }
    println!("{:?}", n_energized.iter().max());
}


fn run_light(grid: &[Vec<char>], start_beam: Beam) -> usize {
    let mut beams: Vec<Beam> = vec![start_beam];
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    energized.insert(start_beam.pos);
    let mut energized_tmp: Vec<usize> = vec![];
    let mut energized_tmp_max: usize = 0;
    loop {
        let mut tmp: Vec<Beam> = vec![];
        for b in beams.iter_mut() {
            tmp.extend(b.run(grid).iter());
        }
        beams = tmp.iter().filter(|b| b.stat == Status::Running).copied().collect();
        beams.iter().for_each(|b| { energized.insert(b.pos); } );
        energized_tmp.push(energized.len());
        if energized_tmp.len() == STOPPING {
            if *energized_tmp.iter().max().unwrap() == energized_tmp_max {
                break;
            } else {
                energized_tmp_max = *energized_tmp.iter().max().unwrap();
                energized_tmp.clear();
            }
        }
    } 
    energized.len()
}