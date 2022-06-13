use std::sync::Arc;

use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub struct Mesh {
    size: u32,
    buffer: wgpu::Buffer,
}

impl Mesh {
    pub fn new(vertex: Vec<Vertex>, device: &Arc<wgpu::Device>) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
            size: vertex.len() as u32,
        }
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.draw(0..self.size, 0..1);
    }

    pub fn destroy(&self) {
        self.buffer.destroy();
    }
}
