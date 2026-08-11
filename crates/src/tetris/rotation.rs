/// Rotation of an object in 4 cardinal directions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    NORTH = 1,
    EAST = 2,
    SOUTH = 3,
    WEST = 4,
}

/// Possible directions that a piece can move
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Rotation {
    /// Set the current rotation to the next position it will be in in one
    /// clockwise rotation
    pub fn clockwise(&mut self) {
        let temp = self.clone();
        *self = match temp {
            Rotation::NORTH => Rotation::EAST,
            Rotation::EAST => Rotation::SOUTH,
            Rotation::SOUTH => Rotation::WEST,
            Rotation::WEST => Rotation::NORTH,
        };
    }

    /// Set the current rotation to the next position it will be in in one
    /// counter clockwise rotation
    pub fn counter_clockwise(&mut self) {
        let temp = self.clone();
        *self = match temp {
            Rotation::NORTH => Rotation::WEST,
            Rotation::EAST => Rotation::NORTH,
            Rotation::SOUTH => Rotation::EAST,
            Rotation::WEST => Rotation::SOUTH,
        }
    }

    pub fn preview_rotation(&self, direction: Direction) -> Self {
        match (*self, direction) {
            (Rotation::NORTH, Direction::Right) => Rotation::EAST,
            (Rotation::EAST, Direction::Right) => Rotation::SOUTH,
            (Rotation::SOUTH, Direction::Right) => Rotation::WEST,
            (Rotation::WEST, Direction::Right) => Rotation::NORTH,
            (Rotation::NORTH, Direction::Left) => Rotation::WEST,
            (Rotation::EAST, Direction::Left) => Rotation::SOUTH,
            (Rotation::SOUTH, Direction::Left) => Rotation::EAST,
            (Rotation::WEST, Direction::Left) => Rotation::NORTH,
        }
    }

    pub fn rotate(&mut self, direction: Direction) {
        *self = self.preview_rotation(direction)
    }
}
