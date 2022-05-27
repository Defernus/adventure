use std::{collections::{self, BTreeMap}, time::{SystemTime, UNIX_EPOCH}};

use wgpu::{ Device, RenderPipeline, SurfaceConfiguration, RenderPass, include_wgsl, util::DeviceExt};

use crate::{utils::{
    position::Position,
}, vertex::Vertex, camera::{Camera, CameraState}, sun::Sun, texture};

use self::{chunk::{Chunk, CHUNK_SIZE}, block::block_handlers::load_block_handlers};

pub mod block;
pub mod chunk;

pub struct World {
    chunks: collections::BTreeMap<Position, Chunk>,
    render_pipeline: RenderPipeline,
    camera: Camera,
    sun: Sun,
}

impl World {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("shaders/main.wgsl"));

        let mut sun = Sun::new(&device);

        let mut camera = Camera::new(
            &device,
            CameraState {
                eye: (0.0, 1.0, 2.0).into(),
                // target: (0., 0., 0.).into(),
                target: (CHUNK_SIZE as f32 / 2., CHUNK_SIZE as f32 / 2., CHUNK_SIZE as f32 / 2.).into(),
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
                bind_group_layouts: &[
                    camera.get_bind_group_layout(),
                    sun.get_bind_group_layout(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
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
        chunks.insert(chunk_pos.clone(), Chunk::generate(chunk_pos.clone(), device));

        return World { sun, chunks, render_pipeline, camera };
    }

    pub fn update(self: &mut Self, queue: &wgpu::Queue) {
        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        });

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let t = since_the_epoch.as_millis() - 1653596661209;
        let t = t as f32 / 1000.;

        let r: f32 = CHUNK_SIZE as f32 * 2.;
        let target = self.camera.state.target;
        self.camera.state.eye = (
            target.x + t.cos() * r,
            target.y + t.cos() * r,
            target.z + t.sin() * r,
        ).into();

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
