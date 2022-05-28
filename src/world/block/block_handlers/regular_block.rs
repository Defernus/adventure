use strum::IntoEnumIterator;

use crate::{
    utils::{direction::Direction, position::Position},
    world::{block::block_data::BlockData, chunk::CHUNK_SIZE},
};

use super::{get_block_handler, helpers::append_face, IBlockHandler};

pub struct RegularBlock {
    is_solid: bool,
    name: &'static str,
    color: Option<[f32; 3]>,
}

impl RegularBlock {
    pub const fn new(is_solid: bool, name: &'static str, color: Option<[f32; 3]>) -> Self {
        return Self {
            is_solid,
            name,
            color,
        };
    }
}

impl RegularBlock {
    fn append_face(
        pos: Position,
        neighbor: BlockData,
        dir: Direction,
        color: [f32; 3],
        vertex: &mut Vec<crate::vertex::Vertex>,
    ) {
        // !TODO check face instead of whole block
        if !get_block_handler(neighbor.block).is_solid(neighbor) {
            append_face::append_face(vertex, pos.clone(), color, dir);
        }
    }
}

impl IBlockHandler for RegularBlock {
    fn get_name(&self, _block_data: BlockData) -> String {
        return self.name.to_string();
    }

    fn is_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }

    fn is_bottom_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_east_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_face_solid(&self, _block_data: BlockData, _face: Direction) -> bool {
        return self.is_solid;
    }
    fn is_north_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_south_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_top_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_west_face_solid(&self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }

    fn update_vertex(&self, block_data: BlockData, vertex: &mut Vec<crate::vertex::Vertex>) {
        match self.color {
            None => {
                return;
            }
            Some(color) => {
                let pos = block_data.in_chunk_position;
                for dir in Direction::iter() {
                    let neighbor_block_data =
                        block_data.chunk.get_neighbor(pos.clone(), dir.clone());
                    let chunk_pos = block_data.chunk.get_position();
                    let face_pos = Position::new(
                        pos.x + chunk_pos.x * CHUNK_SIZE as i64,
                        pos.y + chunk_pos.y * CHUNK_SIZE as i64,
                        pos.z + chunk_pos.z * CHUNK_SIZE as i64,
                    );
                    match neighbor_block_data {
                        Some(neighbor) => {
                            Self::append_face(
                                face_pos.clone(),
                                neighbor,
                                dir.clone(),
                                color,
                                vertex,
                            );
                        }
                        None => {
                            append_face::append_face(vertex, face_pos.clone(), color, dir.clone());
                        }
                    }
                }
            }
        }
    }
}
