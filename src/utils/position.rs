use std::{cmp::Ordering, ops::Add};

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

    pub fn get_east_neighbor(&self) -> Position {
        Position::new(self.x + 1, self.y, self.z)
    }
    pub fn get_west_neighbor(&self) -> Position {
        Position::new(self.x - 1, self.y, self.z)
    }
    pub fn get_up_neighbor(&self) -> Position {
        Position::new(self.x, self.y + 1, self.z)
    }
    pub fn get_down_neighbor(&self) -> Position {
        Position::new(self.x, self.y - 1, self.z)
    }
    pub fn get_north_neighbor(&self) -> Position {
        Position::new(self.x, self.y, self.z + 1)
    }
    pub fn get_south_neighbor(&self) -> Position {
        Position::new(self.x, self.y, self.z - 1)
    }

    pub fn get_neighbor(&self, dir: Direction) -> Position {
        match dir {
            Direction::East => self.get_east_neighbor(),
            Direction::West => self.get_west_neighbor(),
            Direction::Up => self.get_up_neighbor(),
            Direction::Down => self.get_down_neighbor(),
            Direction::North => self.get_north_neighbor(),
            Direction::South => self.get_south_neighbor(),
        }
    }

    pub fn iter_around(&self, radius: usize) -> PositionAroundIterator {
        PositionAroundIterator::new(self.clone(), radius)
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

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
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

#[derive(Clone, Copy)]
pub struct PositionAroundIterator {
    start: Position,
    current: Position,
    current_radius: i64,
    radius: i64,
}

impl PositionAroundIterator {
    pub fn new(start: Position, radius: usize) -> Self {
        Self {
            radius: radius as i64,
            start,
            current_radius: 0,
            current: Position::new(0, -(radius as i64), 0),
        }
    }
}

impl Iterator for PositionAroundIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let r = self.current_radius;
        if self.radius == r {
            return None;
        }

        let y_r = self.radius - r + 1;

        let new_pos = match self.current {
            p if p.y < y_r => p.get_up_neighbor(),
            mut p if p.z == r && p.x == -r => {
                p.y = -y_r + 1;
                self.current_radius += 1;
                p.get_north_neighbor()
            }

            mut p if p.x < r && p.z == r => {
                p.y = -y_r;
                p.get_east_neighbor()
            }

            mut p if p.z > -r && p.x == r => {
                p.y = -y_r;
                p.get_south_neighbor()
            }

            mut p if p.x > -r && p.z == -r => {
                p.y = -y_r;
                p.get_west_neighbor()
            }

            mut p if p.z < r && p.x == -r => {
                p.y = -y_r;
                p.get_north_neighbor()
            }

            _ => {
                panic!("unreachable");
            }
        };

        // new_pos.y = 0;

        self.current = new_pos;

        return Some(new_pos + self.start);
    }
}
