use crate::polygon::shoelace;

pub fn solve(input: &[String]) {
    let mut pos: (i64, i64) = (0, 0);
    let mut vertices: Vec<(i64, i64)> = vec![];
    let mut border: i64 = 0;
    for line in input {
        let spl: Vec<_> = line.split_whitespace().collect();
        let hex = spl[2];
        let steps = i64::from_str_radix(&hex[2..7], 16).unwrap();
        match hex[hex.len()-2..hex.len()-1].chars().next().unwrap() {
            '0' => pos.0 += steps,
            '1' => pos.1 += steps,
            '2' => pos.0 -= steps,
            '3' => pos.1 -= steps,
            _ => panic!("Unknown direction!")
        };
        vertices.push(pos);
        border += steps;
    }
    println!("{:?}", shoelace(&vertices[..vertices.len()-1]) + border / 2 + 1);
}