use crate::polygon::shoelace;

pub fn solve(input: &[String]) {
    let mut pos: (i64, i64) = (0, 0);
    let mut vertices: Vec<(i64, i64)> = vec![];
    let mut border: i64 = 0;
    vertices.push(pos);
    for line in input {
        let spl: Vec<_> = line.split_whitespace().collect();
        let dir = spl[0].chars().next().unwrap();
        let steps = spl[1].parse::<i64>().unwrap();
        match dir {
            'U' => pos.1 -= steps,
            'R' => pos.0 += steps,
            'D' => pos.1 += steps,
            'L' => pos.0 -= steps,
            _ => panic!("Unknown direction!")
        }
        vertices.push(pos);
        border += steps;
    }
    // Solved the first part by using a 2D grid, outlining the shape within the
    // grid and using flood fill function to fill the shape. Unfortunately it's
    // impossible to solve part 2 with this approach. Therefore switched to
    // computing polygon area with shoelace algorithm.
    // Thanks https://www.ericburden.work/blog/2023/12/18/advent-of-code-day-18/
    
    // The area is thus bound by a line running through the center of trench cubes.
    // For side sections of the trench, half of it is inside/outside.
    // added back in. Corner cubes. For the four
    // corners, 3/4 of that cube is outside the bounded region. With four
    // corner cubes, this is an extra +1. The _inner_ corners don't matter
    // because for every cube 3/4 inside the bounded area, there's
    // a corresponding cube with only 1/4 inside the bounded area.
    println!("{:?}", shoelace(&vertices[..vertices.len()-1]) + border / 2 + 1);
}
