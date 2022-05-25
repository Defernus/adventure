use wgpu::{RenderPass, Device, util::DeviceExt};

use crate::{utils::position::Position, vertex::Vertex};

use super::block::Block;

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
    pub fn generate(pos: Position, device: &Device) -> Self {
        let blocks = [Block { id: 0 }; CHUNK_VOLUME];

        let vertex = vec![
            Vertex {
                position: [0., 0., 0.],
                color: [1., 0., 0.],
            },
            Vertex {
                position: [0.5, 0., 0.],
                color: [0., 0., 1.],
            },
            Vertex {
                position: [0., 0.5, 0.],
                color: [0., 1., 0.],
            },
        ];

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
