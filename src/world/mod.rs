use std::collections::{self, BTreeMap};

use wgpu::{include_wgsl, Device, RenderPass, RenderPipeline, SurfaceConfiguration};
use winit::window::Window;

use crate::{
    app_state::game_state::GameSate,
    camera::{state::CameraState, Camera},
    sun::Sun,
    texture,
    utils::position::Position,
    vec::Vec3,
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
    max_chunks: usize,
    max_chunk_dist: f32,
    sun: Sun,
}

impl World {
    pub fn new(window: &Window, device: &Device, config: &SurfaceConfiguration) -> Self {
        let screen_size = window.inner_size();
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
                up: Vec3::new(0., 1., 0.),
                aspect: config.width as f32 / config.height as f32,
                fov_y: 70.0,
                z_near: 0.1,
                z_far: 1024.0,
            },
            (screen_size.width as f32, screen_size.height as f32),
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
            max_chunk_dist: 64.,
            sun,
            chunks,
            max_chunks: 32,
            render_pipeline,
            camera,
        };
    }

    fn load_chunk(&mut self, device: &Device, _game_state: &mut GameSate) -> bool {
        if self.chunks.len() < self.max_chunks {
            let camera_pos = self.camera.state.eye;
            let new_pos = Position::new(
                (camera_pos.x / CHUNK_SIZE as f32) as i64,
                (camera_pos.y / CHUNK_SIZE as f32) as i64,
                (camera_pos.z / CHUNK_SIZE as f32) as i64,
            );

            match self.chunks.get(&new_pos) {
                Some(_) => {
                    return false;
                }
                _ => {}
            }

            let mut new_chunk = Chunk::new(new_pos);
            new_chunk.generate(device);
            self.chunks.insert(new_pos, new_chunk);
            return true;
        }
        return false;
    }

    fn get_chunk_to_unload(&mut self) -> Option<Position> {
        for (pos, _chunk) in self.chunks.iter() {
            let chunk_pos = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32) * CHUNK_SIZE as f32;
            let dist = (chunk_pos - self.camera.state.eye).length();
            if dist > self.max_chunk_dist {
                println!("chunk at {:?} too far and will be unloaded", pos);
                return Some(pos.clone());
            }
        }
        return None;
    }

    fn unload_chunk(&mut self, _game_state: &mut GameSate) -> bool {
        let chunk_to_unload = self.get_chunk_to_unload();
        match chunk_to_unload {
            Some(pos) => {
                self.chunks.remove(&pos);
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, device: &Device, game_state: &mut GameSate) {
        self.load_chunk(device, game_state);

        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        });

        self.camera.update_uniform(queue);
        self.sun.update_uniform(queue);

        self.unload_chunk(game_state);
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
