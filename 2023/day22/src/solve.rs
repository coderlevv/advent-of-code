use std::iter::repeat;
use crate::brick::Brick;

type Dimension = (i32, i32, i32);

pub fn solve(input: &[String]) {
    let mut bricks: Vec<Brick> = vec![];
    for line in input {
        let spl: Vec<_> = line.split('~').collect();
        let coord1: Vec<_> = spl[0].split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let coord2: Vec<_> = spl[1].split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let dim: Dimension = (
            coord2[0] - coord1[0] + 1,
            coord2[1] - coord1[1] + 1,
            coord2[2] - coord1[2] + 1
        );
        let pos = (coord1[0], coord1[1], coord1[2]);
        let brick = Brick::new(pos, dim);
        bricks.push(brick);
    }

    settle(&mut bricks);
    let mut disint: u32 = 0;
    for idx in 0..bricks.len() {
        let mut tmp = bricks.clone();
        tmp.remove(idx);
        // let num= settle(&mut tmp);
        // let num_fallen = num.iter().filter(|n| **n > 0).count();
        // if num_fallen == 0 {
        if !settle(&mut tmp) {
            disint += 1;
        }
    }
    println!("{:?}", disint);

    let mut num_fall: Vec<usize> = vec![];
    for idx in 0..bricks.len() {
        let mut tmp = bricks.clone();
        tmp.remove(idx);
        let num= settle_iter(&mut tmp);
        let num_fallen = num.iter().filter(|n| **n > 0).count();
        num_fall.push(num_fallen);
    }
    println!("{:?}", num_fall.iter().sum::<usize>());
}
    
pub fn settle(bricks: &mut [Brick]) -> bool {
    let mut has_changed: bool = false;
    loop {
        let num = settle_iter(bricks);
        let num_fallen = num.iter().filter(|n| **n > 0).count();
        if num_fallen > 0 { has_changed = true; }
        else { break; }
    }
    has_changed
}

pub fn settle_iter(bricks: &mut [Brick]) -> Vec<u32> {
    let mut fallen: Vec<u32> = Vec::from_iter(repeat(0).take(bricks.len()));
    bricks.sort_by(|a, b| a.pos.2.cmp(&b.pos.2));
    for idx in 0..bricks.len() {
        let (lower, upper) = bricks.split_at_mut(idx);
        let brick = &mut upper[0];
        // nothing to do, if brick is already at z=1
        if brick.pos.2 == 1 { continue; }
        let mut clear_below = true;
        for other in lower {
            if !other.pos.2 >= brick.pos.2 { continue; } 
            // look below brick on x axis
            for x in 0..brick.dim.0 {
                if other.contains((brick.pos.0+x, brick.pos.1, brick.pos.2 - 1)) {
                    clear_below = false;
                }
            }
            // look below brick on y axis
            for y in 0..brick.dim.1 {
                if other.contains((brick.pos.0, brick.pos.1+y, brick.pos.2 - 1)) {
                    clear_below = false;
                }
            }
        }
        if clear_below && brick.pos.2 > 1 {
            brick.pos.2 -= 1;
            fallen[idx] += 1;
        }
    }
    fallen
}