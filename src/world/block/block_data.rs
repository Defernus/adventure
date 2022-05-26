use crate::utils::position::Position;

use super::Block;

pub struct BlockData {
    pub block: Block,
    pub in_chunk_position: Position,
}
