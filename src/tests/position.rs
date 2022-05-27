use crate::utils::{direction::Direction, position::Position};

#[test]
fn get_neighbor_up() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::Up);

    let expected = Position::new(pos.x, pos.y + 1, pos.z);

    assert_eq!(expected, neighbor);
}

#[test]
fn get_neighbor_down() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::Down);

    let expected = Position::new(pos.x, pos.y - 1, pos.z);

    assert_eq!(expected, neighbor);
}

#[test]
fn get_neighbor_west() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::West);

    let expected = Position::new(pos.x - 1, pos.y, pos.z);

    assert_eq!(expected, neighbor);
}

#[test]
fn get_neighbor_east() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::East);

    let expected = Position::new(pos.x + 1, pos.y, pos.z);

    assert_eq!(expected, neighbor);
}

#[test]
fn get_neighbor_south() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::South);

    let expected = Position::new(pos.x, pos.y, pos.z - 1);

    assert_eq!(expected, neighbor);
}

#[test]
fn get_neighbor_north() {
    let pos = Position::new(0, 0, 0);
    let neighbor = pos.get_neighbor(Direction::North);

    let expected = Position::new(pos.x, pos.y, pos.z + 1);

    assert_eq!(expected, neighbor);
}
