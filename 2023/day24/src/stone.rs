use assert_approx_eq::assert_approx_eq;

#[derive(Debug)]
pub struct Stone {
    pub pos: (f64, f64, f64),
    pub vel: (f64, f64, f64)
}

impl Stone {
    pub fn from_str(line: &str) -> Self {
        let spl: Vec<_> = line.split('@').collect();
        let pos: Vec<f64> = spl[0]
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap() as f64)
            .collect();
        let vel: Vec<f64> = spl[1]
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap() as f64)
            .collect();
        Stone {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2])
        }
    }

    pub fn in_the_past(&self, x: &f64) -> bool {
        match self.vel.0 > 0.0 {
            true => *x < self.pos.0,
            false => *x > self.pos.0
        }
    }

    pub fn intersection_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let a1 = self.vel.1;
        let b1 = -self.vel.0;
        let c1 = a1 * self.pos.0 + b1 * self.pos.1;
        let a2 = other.vel.1;
        let b2 = -other.vel.0;
        let c2 = a2 * other.pos.0 + b2 * other.pos.1;
        let det = a1 * b2 - a2 * b1;
        if det == 0.0 {
            None
        } else {
            let x_inter = ((b2 * c1) - (b1 * c2)) / det;
            let y_inter = ((a1 * c2) - (a2 * c1)) / det;
            Some((x_inter, y_inter))
        }
    }
}

#[test]
fn intersection_2d_test() {
    // Hailstone A: 19, 13, 30 @ -2, 1, -2
    // Hailstone B: 18, 19, 22 @ -1, -1, -2
    let s1 = Stone { pos: (19.0, 13.0, 30.0), vel: (-2.0, 1.0, -2.0) };
    let s2 = Stone { pos: (18.0, 19.0, 22.0), vel: (-1.0, -1.0, -2.0) };
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().0, 14.333333);
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().1, 15.333333);

    // Hailstone A: 19, 13, 30 @ -2, 1, -2
    // Hailstone B: 20, 25, 34 @ -2, -2, -4
    let s1 = Stone { pos: (19.0, 13.0, 30.0), vel: (-2.0, 1.0, -2.0) };
    let s2 = Stone { pos: (20.0, 25.0, 34.0), vel: (-2.0, -2.0, -4.0) };
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().0, 11.666666);
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().1, 16.666666);

    // Hailstone A: 18, 19, 22 @ -1, -1, -2
    // Hailstone B: 20, 25, 34 @ -2, -2, -4
    let s1 = Stone { pos: (18.0, 19.0, 22.0), vel: (-1.0, -1.0, -2.0) };
    let s2 = Stone { pos: (20.0, 25.0, 34.0), vel: (-2.0, -2.0, -4.0) };
    assert_eq!(s1.intersection_2d(&s2), None);

    // Hailstone A: 18, 19, 22 @ -1, -1, -2
    // Hailstone B: 12, 31, 28 @ -1, -2, -1
    let s1 = Stone { pos: (18.0, 19.0, 22.0), vel: (-1.0, -1.0, -2.0) };
    let s2 = Stone { pos: (12.0, 31.0, 28.0), vel: (-1.0, -2.0, -1.0) };
    assert_eq!(s1.intersection_2d(&s2), Some((-6.0, -5.0)));

    // Hailstone A: 18, 19, 22 @ -1, -1, -2
    // Hailstone B: 20, 19, 15 @ 1, -5, -3
    let s1 = Stone { pos: (18.0, 19.0, 22.0), vel: (-1.0, -1.0, -2.0) };
    let s2 = Stone { pos: (20.0, 19.0, 15.0), vel: (1.0, -5.0, -3.0) };
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().0, 19.666666);
    assert_approx_eq!(s1.intersection_2d(&s2).unwrap().1, 20.666666);


}