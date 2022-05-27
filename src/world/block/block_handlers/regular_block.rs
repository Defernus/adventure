use strum::IntoEnumIterator;

use crate::{
    utils::{direction::Direction, position::Position},
    world::block::block_data::BlockData,
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
    fn get_name(self: &Self, _block_data: BlockData) -> String {
        return self.name.to_string();
    }

    fn is_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }

    fn is_bottom_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_east_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_face_solid(self: &Self, _block_data: BlockData, _face: Direction) -> bool {
        return self.is_solid;
    }
    fn is_north_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_south_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_top_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_west_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }

    fn update_vertex(self: &Self, block_data: BlockData, vertex: &mut Vec<crate::vertex::Vertex>) {
        match self.color {
            None => {
                return;
            }
            Some(color) => {
                let pos = block_data.in_chunk_position;
                for dir in Direction::iter() {
                    let block_data = block_data.chunk.get_neighbor(pos.clone(), dir.clone());
                    match block_data {
                        Some(neighbor) => {
                            Self::append_face(pos.clone(), neighbor, dir.clone(), color, vertex);
                        }
                        None => {
                            append_face::append_face(vertex, pos.clone(), color, dir.clone());
                        }
                    }
                }
            }
        }
    }
}
