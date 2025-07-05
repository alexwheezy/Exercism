// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, dir: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        use Direction::*;
        match self.dir {
            North => Self { dir: East, ..self },
            East => Self { dir: South, ..self },
            South => Self { dir: West, ..self },
            West => Self { dir: North, ..self },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        use Direction::*;
        match self.dir {
            North => Self { dir: West, ..self },
            East => Self { dir: North, ..self },
            South => Self { dir: East, ..self },
            West => Self { dir: South, ..self },
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn advance(self) -> Self {
        use Direction::*;
        match self.dir {
            North => Self { y: self.y + 1, ..self },
            East => Self { x: self.x + 1, ..self },
            South => Self { y: self.y - 1, ..self },
            West => Self { x: self.x - 1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instr in instructions.chars() {
            robot = match instr {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                _ => robot.advance(),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
