use noise::NoiseFn;
use wgpu::{RenderPass, Device, util::DeviceExt};

use crate::{utils::position::Position, vertex::Vertex};

use super::block::{Block, BlockId, block_data::BlockData, block_handlers::get_block_handler};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    biome: i32,
    pos: Position,
    vertex: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    blocks: [Block; CHUNK_VOLUME],
}

impl Chunk {
    fn generate_vertex(blocks: &[Block; CHUNK_VOLUME]) -> Vec<Vertex> {
        let mut vertex: Vec<Vertex> = Vec::new();
        for i in 0..CHUNK_VOLUME {
            let handler = get_block_handler(blocks[i]);
            let block_data = BlockData {
                block: blocks[i].clone(),
                in_chunk_position: Position {
                    x: (i % CHUNK_SIZE) as i64,
                    y: ((i / CHUNK_SIZE) % CHUNK_SIZE) as i64,
                    z: (i / CHUNK_SIZE / CHUNK_SIZE) as i64,
                },
            };
            // println!("i: {}, id: {}", i, blocks[i].id);
            handler.update_vertex(blocks, block_data, &mut vertex);
        }

        return vertex;
    }

    pub fn generate(pos: Position, device: &Device) -> Self {
        println!("generating chung {}, {}, {}", pos.x, pos.y, pos.z);
        let perlin = noise::Perlin::new();
        let mut blocks = [Block { id: 0 }; CHUNK_VOLUME];
        for i in 0..CHUNK_VOLUME {
            let noise_v = perlin.get([i as f64 * 0.1, 0., 0.]) * 2.;
            blocks[i].id = (noise_v) as BlockId;
        }
        blocks[0].id = 1;

        let vertex = Self::generate_vertex(&blocks);
        // println!("vertex: {}", vertex.len());
        // for v in vertex.iter() {
        //     println!("v: {}, {}, {}; c: {}, {}, {}", v.position[0], v.position[1], v.position[2], v.color[0], v.color[1], v.color[2]);
        // }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );


        return Self {
            biome: 0,
            pos: pos.clone(),
            blocks,
            vertex,

            vertex_buffer,
        };
    }

    pub fn update(self: &Self) {

    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex.len() as u32, 0..1);
    }
}
