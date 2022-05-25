use std::collections::{self, BTreeMap};

use wgpu::{ Device, RenderPipeline, SurfaceConfiguration, RenderPass, include_wgsl};

use crate::{utils::{
    position::Position,
}, vertex::Vertex};

use self::{chunk::Chunk};

pub mod block;
pub mod chunk;

pub struct World {
    chunks: collections::BTreeMap<Position, Chunk>,
    render_pipeline: RenderPipeline
}

impl World {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("shaders/main.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::getDescription(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut chunks = BTreeMap::new();
        let chunk_pos = Position { x: 0, y: 0, z: 0 };
        chunks.insert(chunk_pos.clone(), Chunk::generate(chunk_pos.clone(), device));

        World { chunks, render_pipeline }
    }

    pub fn update(self: &mut Self) {
        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        })
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        for (_pos, chunk) in self.chunks.iter() {
            chunk.draw(render_pass);
        }
    }
}
