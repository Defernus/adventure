use crate::{utils::position::Position, world::chunk::Chunk};

use super::Voxel;

pub struct VoxelData<'a> {
    pub chunk: &'a Chunk,
    pub voxel: Voxel,
    pub in_chunk_position: Position,
}
