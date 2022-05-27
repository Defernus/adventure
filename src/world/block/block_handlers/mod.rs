use crate::{utils::direction::Direction, vertex::Vertex};

use self::regular_block::RegularBlock;

use super::{block_data::BlockData, Block};

pub mod helpers;
pub mod regular_block;

pub trait IBlockHandler {
    fn get_name(self: &Self, block_data: BlockData) -> String;
    fn is_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_face_solid(self: &Self, block_data: BlockData, face: Direction) -> bool;

    fn is_top_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_bottom_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_north_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_south_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_west_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_east_face_solid(self: &Self, block_data: BlockData) -> bool;

    fn update_vertex(self: &Self, block_data: BlockData, vertex: &mut Vec<Vertex>);
}

const BLOCK_HANDLERS: [&'static dyn IBlockHandler; 2] = [
    &RegularBlock::new(false, "block.air", None),
    &RegularBlock::new(true, "block.stone", Some([0.5, 0.5, 0.5])),
];

pub fn load_block_handlers() {}

pub fn get_block_handler(block: Block) -> &'static dyn IBlockHandler {
    if BLOCK_HANDLERS.len() <= block.id as usize {
        panic!("unexpected block id: {}", block.id);
    }
    return BLOCK_HANDLERS[block.id as usize];
}
