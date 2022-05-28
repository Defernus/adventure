use std::cmp::Ordering;

use super::direction::Direction;

#[derive(Clone, Copy, Eq, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        return Position { x, y, z };
    }

    pub fn get_neighbor(&self, dir: Direction) -> Position {
        match dir {
            Direction::East => Position::new(self.x + 1, self.y, self.z),
            Direction::West => Position::new(self.x - 1, self.y, self.z),
            Direction::Up => Position::new(self.x, self.y + 1, self.z),
            Direction::Down => Position::new(self.x, self.y - 1, self.z),
            Direction::North => Position::new(self.x, self.y, self.z + 1),
            Direction::South => Position::new(self.x, self.y, self.z - 1),
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater;
        }
        if self.x < other.x {
            return Ordering::Less;
        }
        if self.y > other.y {
            return Ordering::Greater;
        }
        if self.y < other.y {
            return Ordering::Less;
        }
        if self.z > other.z {
            return Ordering::Greater;
        }
        if self.z < other.z {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
