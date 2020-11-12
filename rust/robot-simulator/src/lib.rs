// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            d: match self.d {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Self {
            x: match self.d {
                Direction::West => self.x - 1,
                Direction::East => self.x + 1,
                _ => self.x,
            },
            y: match self.d {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y,
            },
            d: self.d,
        }
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars() {
            self = match i {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => self,
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
