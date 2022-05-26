use crate::{vertex::Vertex, world::{block::{block_data::BlockData, Block}, chunk::CHUNK_VOLUME}, utils::direction::Direction};

use super::IBlockHandler;

pub struct RegularBlock {
    is_solid: bool,
    name: &'static str,
    color: Option<[f32; 3]>,
}

impl RegularBlock {
    pub const fn new(is_solid: bool, name: &'static str, color: Option<[f32; 3]>) -> Self {
        return Self { is_solid, name, color };
    }
}

impl IBlockHandler for RegularBlock {
    fn is_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    
    fn is_bottom_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_east_face_solid(self: &Self, _block_data: BlockData) -> bool {
        return self.is_solid;
    }
    fn is_face_solid(self: &Self, _block_data: BlockData, face: Direction) -> bool {
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
    
    fn update_vertex(self: &Self, blocks: &[Block; CHUNK_VOLUME], block_data: BlockData, vertex: &mut Vec<crate::vertex::Vertex>) {
        match self.color {
            None => {
                return;
            },
            Some(color) => {
                let pos = block_data.in_chunk_position;
                let x = pos.x as f32;
                let y = pos.y as f32;
                let z = pos.z as f32;
                // println!("block {} {} {}", x, y, z);
                
                vertex.push(Vertex { position: [x, y, z], color });
                vertex.push(Vertex { position: [x + 1., y, z], color });
                vertex.push(Vertex { position: [x + 1., y + 1., z], color });
                
                vertex.push(Vertex { position: [x, y, z], color });
                vertex.push(Vertex { position: [x + 1., y + 1., z], color });
                vertex.push(Vertex { position: [x, y + 1., z], color });
            }
        }
    }
}