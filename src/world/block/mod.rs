use crate::utils::direction::Direction;

use self::block_data::BlockData;

pub mod block_data;


#[derive(Clone, Copy)]
pub struct Block {
    pub id: i32,
}

pub trait IBlock {
    fn is_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_face_solid(self: &Self, block_data: BlockData, face: Direction) -> bool;

    fn is_top_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_bottom_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_north_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_south_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_west_face_solid(self: &Self, block_data: BlockData) -> bool;
    fn is_east_face_solid(self: &Self, block_data: BlockData) -> bool;
}
