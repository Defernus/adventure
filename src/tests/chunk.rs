use crate::{
    utils::{direction::Direction, position::Position},
    world::{
        block::{block_handlers::get_block_handler, Block},
        chunk::Chunk,
    },
};

#[test]
fn get_set_and_get_block() {
    let pos = Position::new(1, 3, 7);
    let mut chunk = Chunk::new(Position::new(0, 0, 0));

    let block = chunk.get_block(pos.clone()).expect("0 block not found");
    assert_eq!(block.block.id, 0);

    chunk.set_block(pos.clone(), Block { value: 1., id: 1 });
    let block = chunk.get_block(pos.clone()).expect("1 block not found");
    assert_eq!(block.block.id, 1);
}

#[test]
fn get_neighbor() {
    let pos = Position::new(0, 0, 0);
    let mut chunk = Chunk::new(Position::new(0, 0, 0));
    let neighbor_pos = pos.get_neighbor(Direction::East);

    chunk.set_block(neighbor_pos.clone(), Block { value: 1., id: 1 });
    let block = chunk
        .get_neighbor(pos, Direction::East)
        .expect("neighbor not found");

    assert_eq!(block.block.id, 1);
}

#[test]
fn check_if_solid() {
    let pos = Position::new(0, 0, 0);
    let mut chunk = Chunk::new(Position::new(0, 0, 0));
    chunk.set_block(pos.clone(), Block { value: 1., id: 1 });

    let block_data = chunk.get_block(pos.clone()).expect("block not found");

    let handler = get_block_handler(block_data.block);

    assert_eq!(handler.is_solid(block_data), true);
}

#[test]
fn check_if_not_solid() {
    let pos = Position::new(0, 0, 0);
    let chunk = Chunk::new(Position::new(0, 0, 0));

    let block_data = chunk.get_block(pos.clone()).expect("block not found");
    let handler = get_block_handler(block_data.block);

    assert_eq!(handler.is_solid(block_data), false);
}

// #[test]
// fn chunk_get_neighbor_block() {
//     let chunk = Chunk::new(Position::new(0, 0, 0));
// }
