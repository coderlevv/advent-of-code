type Position = (i32 ,i32, i32);
type Dimension = (i32, i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Brick {
    pub pos: Position,
    pub dim: Dimension
}

impl Brick {
    pub fn new(pos: Position, dim: Dimension) -> Self {
        Self {
            pos,
            dim
        }
    }

    pub fn contains(&self, pos: Position) -> bool {
        let mut within = false;
        for x in 0..self.dim.0 {
            if (self.pos.0 + x, self.pos.1, self.pos.2) == pos {
                within = true;
                break;
            }
        }
        if !within {
            for y in 0..self.dim.1 {
                if (self.pos.0, self.pos.1 + y, self.pos.2) == pos {
                    within = true;
                    break;
                }
            }
        }
        if !within {
            for z in 0..self.dim.2 {
                if (self.pos.0, self.pos.1, self.pos.2 + z) == pos {
                    within = true;
                    break;
                }
            }
        }
        within
    }
}

#[test]
pub fn contains_test () {
    let brick = Brick { pos: (1, 0, 1), dim: (1, 3, 1) };
    assert!(brick.contains((1,0,1)));
    assert!(brick.contains((1,1,1)));
    assert!(brick.contains((1,2,1)));
    assert!(!brick.contains((1,3,1)));
    assert!(!brick.contains((1,0,2)));
}