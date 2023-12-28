#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up, Down, Right, Left
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Starting,
    Running,
    Done
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Beam {
    pub start: (usize, usize),
    pub pos: (usize, usize),
    dir: Direction,
    pub stat: Status
}

impl Beam {
    pub fn new(start: (usize, usize), dir: Direction) -> Self {
        Self {
            start,
            pos: start,
            dir,
            stat: Status::Starting
        }
    }

    pub fn run(&mut self, grid: &[Vec<char>]) -> Vec<Beam> {
        let mut res: Vec<Beam> = vec![];
        let x = self.pos.0;
        let y = self.pos.1;
        match self.dir {
            Direction::Right => {
                if x < grid[0].len()-1 || self.stat == Status::Starting {
                    let mut nxt = grid[y][x+1];
                    if self.stat == Status::Starting {
                        nxt = grid[self.start.1][self.start.0];
                        //self.stat = Status::Running
                    }
                    match nxt {
                        '.' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x+1, y);
                            }
                            res.push(*self);
                        },
                        '-' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x+1, y);
                            }
                            res.push(*self);
                        },
                        '/' => {
                            self.dir = Direction::Up;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x+1, y);
                            }
                            res.push(*self);
                        },
                        '\\' => {
                            self.dir = Direction::Down;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x+1, y);
                            }
                            res.push(*self);
                        },
                        '|' => {
                            self.dir = Direction::Down;
                            if self.stat != Status::Starting {
                                self.pos = (x+1, y);
                            }
                            let mut new_beam = Beam::new(self.start, Direction::Up);
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                new_beam.pos = (x+1, y);
                            }
                            new_beam.stat = Status::Running;
                            res.push(*self);
                            res.push(new_beam);
                        },
                        _ => panic!("Unknown object!")
                    }
                } else {
                    self.stat = Status::Done;
                    res.push(*self)
                }
            },
            Direction::Left => {
                if x > 0 || self.stat == Status::Starting {
                    let mut nxt = grid[y][x-1];
                    if self.stat == Status::Starting {
                        nxt = grid[self.start.1][self.start.0];
                        self.stat = Status::Running
                    }
                    match nxt {
                        '.' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x-1, y);
                            }
                            res.push(*self);
                        },
                        '-' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x-1, y);
                            }
                            res.push(*self);
                        },
                        '/' => {
                            self.dir = Direction::Down;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x-1, y);
                            }
                            res.push(*self);
                        },
                        '\\' => {
                            self.dir = Direction::Up;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x-1, y);
                            }
                            res.push(*self);
                        },
                        '|' => {
                            self.dir = Direction::Up;
                            if self.stat != Status::Starting {
                                self.pos = (x-1, y);
                            }
                            let mut new_beam = Beam::new(self.start, Direction::Down); 
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                new_beam.pos = (x-1, y);
                            }
                            new_beam.stat = Status::Running;
                            res.push(*self);
                            res.push(new_beam);
                        },
                        _ => panic!("Unknown object!")
                    }
                } else {
                    self.stat = Status::Done;
                    res.push(*self)
                }
            },
            Direction::Up => {
                if y > 0 || self.stat == Status::Starting {
                    let mut nxt = grid[y-1][x];
                    if self.stat == Status::Starting {
                        nxt = grid[self.start.1][self.start.0];
                        self.stat = Status::Running
                    }
                    match nxt {
                        '.' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y-1);
                            }
                            res.push(*self);
                        },
                        '|' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y-1);
                            }
                            res.push(*self);
                        },
                        '/' => {
                            self.dir = Direction::Right;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y-1);
                            }
                            res.push(*self);
                        },
                        '\\' => {
                            self.dir = Direction::Left;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y-1);
                            }
                            res.push(*self);
                        },
                        '-' => {
                            self.dir = Direction::Left;
                            if self.stat != Status::Starting {
                                self.pos = (x, y-1);
                            }
                            let mut new_beam = Beam::new(self.start, Direction::Right);
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                new_beam.pos = (x, y-1);
                            }
                            new_beam.stat = Status::Running;
                            res.push(*self);
                            res.push(new_beam);
                        },
                        _ => panic!("Unknown object!")
                    }
                } else {
                    self.stat = Status::Done;
                    res.push(*self)
                }
            },
            Direction::Down => {
                if y < grid.len()-1 || self.stat == Status::Starting {
                    let mut nxt = grid[y+1][x];
                    if self.stat == Status::Starting {
                        nxt = grid[self.start.1][self.start.0];
                        self.stat = Status::Running
                    }
                    match nxt {
                        '.' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y+1);
                            }
                            res.push(*self);
                        },
                        '|' => {
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y+1);
                            }
                            res.push(*self);
                        },
                        '/' => {
                            self.dir = Direction::Left;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y+1);
                            }
                            res.push(*self);
                        },
                        '\\' => {
                            self.dir = Direction::Right;
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                self.pos = (x, y+1);
                            }
                            res.push(*self);
                        },
                        '-' => {
                            self.dir = Direction::Right;
                            if self.stat != Status::Starting {
                                self.pos = (x, y+1);
                            }
                            let mut new_beam = Beam::new(self.start, Direction::Left);
                            if self.stat == Status::Starting {
                                self.stat = Status::Running;
                            } else {
                                new_beam.pos = (x, y+1);
                            }
                            new_beam.stat = Status::Running;
                            res.push(*self);
                            res.push(new_beam);
                        },
                        _ => panic!("Unknown object!")
                    }
                } else {
                    self.stat = Status::Done;
                    res.push(*self)
                }
            },

        }
        res
    }

}

