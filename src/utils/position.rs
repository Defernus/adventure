use std::cmp::Ordering;

#[derive(Clone, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
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
