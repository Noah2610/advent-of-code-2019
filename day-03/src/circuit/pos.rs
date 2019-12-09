use std::fmt;

#[derive(PartialEq, Eq, Hash, Default, Clone, Copy, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn calc_manhattan_dist(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
