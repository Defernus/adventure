use std::collections::{self, BTreeMap};

use wgpu::{include_wgsl, Device, RenderPass, RenderPipeline, SurfaceConfiguration};

use crate::{
    app_state::game_state::GameSate,
    camera::{state::CameraState, Camera},
    sun::Sun,
    texture,
    utils::position::Position,
    vertex::Vertex,
};

use self::{
    block::block_handlers::load_block_handlers,
    chunk::{Chunk, CHUNK_SIZE},
};

pub mod block;
pub mod chunk;

pub struct World {
    chunks: collections::BTreeMap<Position, Chunk>,
    render_pipeline: RenderPipeline,
    pub camera: Camera,
    sun: Sun,
}

impl World {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("shaders/main.wgsl"));

        let sun = Sun::new(&device);

        let camera = Camera::new(
            &device,
            CameraState {
                eye: (8.0, 8.0, -32.0).into(),
                target: (
                    CHUNK_SIZE as f32 / 2.,
                    CHUNK_SIZE as f32 / 2.,
                    CHUNK_SIZE as f32 / 2.,
                )
                    .into(),
                up: cgmath::Vector3::unit_y(),
                aspect: config.width as f32 / config.height as f32,
                fov_y: 45.0,
                z_near: 0.1,
                z_far: 100.0,
            },
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera.get_bind_group_layout(), sun.get_bind_group_layout()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::get_description()],
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
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

        let mut first_chunk = Chunk::new(chunk_pos.clone());
        first_chunk.generate(device);

        chunks.insert(chunk_pos.clone(), first_chunk);

        return World {
            sun,
            chunks,
            render_pipeline,
            camera,
        };
    }

    pub fn update(&mut self, queue: &wgpu::Queue, _game_state: &GameSate) {
        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        });

        self.camera.update_uniform(queue);
        self.sun.update_uniform(queue);
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, self.camera.get_bind_group(), &[]);
        render_pass.set_bind_group(1, self.sun.get_bind_group(), &[]);

        for (_pos, chunk) in self.chunks.iter() {
            chunk.draw(render_pass);
        }
    }
}
