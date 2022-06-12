use noise::NoiseFn;
use wgpu::{util::DeviceExt, Device, RenderPass};

use crate::{
    utils::{direction::Direction, position::Position},
    vertex::Vertex,
};

use super::block::{block_data::BlockData, block_handlers::get_block_handler, Block};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    pos: Position,
    vertex_data: Option<ChunkVertex>,
    blocks: [Block; CHUNK_VOLUME],
}

struct ChunkVertex {
    vertex: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
}

impl Chunk {
    pub fn new(pos: Position) -> Self {
        Self {
            pos: pos,
            blocks: [Block { value: 1., id: 0 }; CHUNK_VOLUME],
            vertex_data: None,
        }
    }

    fn generate_vertex(&mut self) -> Vec<Vertex> {
        let mut vertex: Vec<Vertex> = Vec::new();
        for i in 0..CHUNK_VOLUME {
            let handler = get_block_handler(self.blocks[i]);
            let block_data = BlockData {
                chunk: self,
                block: self.blocks[i].clone(),
                in_chunk_position: Position {
                    x: (i % CHUNK_SIZE) as i64,
                    y: ((i / CHUNK_SIZE) % CHUNK_SIZE) as i64,
                    z: (i / CHUNK_SIZE / CHUNK_SIZE) as i64,
                },
            };
            handler.update_vertex(block_data, &mut vertex);
        }

        return vertex;
    }

    pub fn generate(&mut self, device: &Device) {
        // println!("generating chunk {:?}", self.pos);

        let simplex = noise::OpenSimplex::new();
        let noise_scale = 0.03;
        let noise_threshold: f32 = 0.4;
        for i in 0..CHUNK_VOLUME {
            let pos = Self::index_to_pos(i);

            let x = (self.pos.x * CHUNK_SIZE as i64 + pos.x) as f64;
            let y = (self.pos.y * CHUNK_SIZE as i64 + pos.y) as f64;
            let z = (self.pos.z * CHUNK_SIZE as i64 + pos.z) as f64;

            let mut noise_v = simplex.get([x * noise_scale, y * noise_scale, z * noise_scale]);
            noise_v += 1.0;
            noise_v /= 2.0;

            noise_v *= 1.0 - (y / CHUNK_SIZE as f64).min(1.0).max(0.0);

            if noise_v < noise_threshold.into() {
                self.blocks[i].id = 0;
            } else {
                self.blocks[i].id = 1;
                self.blocks[i].value = (noise_v as f32 - noise_threshold) / (1. - noise_threshold);
            };
        }

        let vertex = self.generate_vertex();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_data = Some(ChunkVertex {
            vertex,
            vertex_buffer,
        });
    }

    pub fn pos_to_index(pos: Position) -> Option<usize> {
        if !Self::check_pos_in_chunk(pos) {
            return None;
        }
        return Some(
            pos.x as usize + pos.y as usize * CHUNK_SIZE + pos.z as usize * CHUNK_SIZE * CHUNK_SIZE,
        );
    }

    pub fn index_to_pos(index: usize) -> Position {
        return Position {
            x: (index % CHUNK_SIZE) as i64,
            y: ((index / CHUNK_SIZE) % CHUNK_SIZE) as i64,
            z: (index / CHUNK_SIZE / CHUNK_SIZE) as i64,
        };
    }

    pub fn pos_to_relative(&self, pos: Position) -> Position {
        return Position::new(
            pos.x - self.pos.x * CHUNK_SIZE as i64,
            pos.y - self.pos.y * CHUNK_SIZE as i64,
            pos.y - self.pos.y * CHUNK_SIZE as i64,
        );
    }

    pub fn set_block(&mut self, in_chunk_position: Position, block: Block) -> bool {
        match Self::pos_to_index(in_chunk_position) {
            Some(index) => {
                self.blocks[index] = block;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn get_block(&self, in_chunk_position: Position) -> Option<BlockData> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                let block = self.blocks[index];
                return Some(BlockData {
                    block,
                    chunk: self,
                    in_chunk_position,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn set_pos(&self, in_chunk_position: Position) -> Option<BlockData> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                let block = self.blocks[index];
                return Some(BlockData {
                    block,
                    chunk: self,
                    in_chunk_position,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn check_pos_in_chunk(pos: Position) -> bool {
        return pos.x >= 0
            && pos.x < CHUNK_SIZE as i64
            && pos.y >= 0
            && pos.y < CHUNK_SIZE as i64
            && pos.z >= 0
            && pos.z < CHUNK_SIZE as i64;
    }

    pub fn get_neighbor(&self, pos: Position, dir: Direction) -> Option<BlockData> {
        let result_pos = pos.get_neighbor(dir);

        match Self::pos_to_index(result_pos.clone()) {
            Some(index) => {
                return Some(BlockData {
                    block: self.blocks[index],
                    in_chunk_position: result_pos.clone(),
                    chunk: &self,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn update(&self) {}

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        match self.vertex_data.as_ref() {
            Some(vertex_data) => {
                render_pass.set_vertex_buffer(0, vertex_data.vertex_buffer.slice(..));
                render_pass.draw(0..vertex_data.vertex.len() as u32, 0..1);
            }
            _ => {}
        }
    }

    pub fn get_position(&self) -> Position {
        return self.pos;
    }
}
