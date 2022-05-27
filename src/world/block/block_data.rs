use crate::{utils::position::Position, world::chunk::Chunk};

use super::Block;

pub struct BlockData<'a> {
    pub chunk: &'a Chunk,
    pub block: Block,
    pub in_chunk_position: Position,
}
