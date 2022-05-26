use std::collections::{self, BTreeMap};

use wgpu::{ Device, RenderPipeline, SurfaceConfiguration, RenderPass, include_wgsl, util::DeviceExt};

use crate::{utils::{
    position::Position,
}, vertex::Vertex, camera::{Camera, CameraUniform}};

use self::{chunk::Chunk, block::block_handlers::load_block_handlers};

pub mod block;
pub mod chunk;

pub struct World {
    chunks: collections::BTreeMap<Position, Chunk>,
    render_pipeline: RenderPipeline,
    camera: Camera,
}

impl World {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("shaders/main.wgsl"));

        let camera = Camera::new(
            &device,
            (0.0, 1.0, 2.0).into(),
            (0.0, 0.0, 0.0).into(),
            cgmath::Vector3::unit_y(),
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            100.0,
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    camera.get_bind_group_layout(),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::get_description(),
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

        load_block_handlers();

        let mut chunks = BTreeMap::new();
        let chunk_pos = Position { x: 0, y: 0, z: 0 };
        chunks.insert(chunk_pos.clone(), Chunk::generate(chunk_pos.clone(), device));

        return World { chunks, render_pipeline, camera };
    }

    pub fn update(self: &mut Self) {
        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        })
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, self.camera.get_bind_group(), &[]);
        for (_pos, chunk) in self.chunks.iter() {
            chunk.draw(render_pass);
        }
    }
}
