use std::collections::{self, BTreeMap};

use wgpu::{include_wgsl, Device, RenderPass, RenderPipeline, SurfaceConfiguration};
use winit::window::Window;

use crate::{
    app_state::game_state::GameSate,
    camera::{state::CameraState, Camera},
    sun::Sun,
    texture,
    utils::position::{Position, PositionAroundIterator},
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
    render_distance: usize,

    prev_player_chunk: Position,
    chunk_load_iterator: PositionAroundIterator,

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
                eye: (8.0, 64., -16.0 * 18.0 + 0.7).into(),
                target: (0.8, -64., 0.).into(),
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

        let render_distance = 8;

        return World {
            sun,
            chunks,
            render_distance,
            render_pipeline,
            camera,
            prev_player_chunk: Position::new(0, 0, 0),
            chunk_load_iterator: Position::new(0, 0, 0).iter_around(render_distance),
        };
    }

    fn get_max_chunk(&self) -> usize {
        (self.render_distance * 2 + 1).pow(3)
    }

    fn load_chunk(&mut self, device: &Device, _game_state: &mut GameSate) -> bool {
        if self.chunks.len() >= self.get_max_chunk() {
            return false;
        }

        let camera_pos = self.camera.state.eye;
        let player_chunk_pos = Position::new(
            (camera_pos.x / CHUNK_SIZE as f32) as i64,
            (camera_pos.y / CHUNK_SIZE as f32) as i64,
            (camera_pos.z / CHUNK_SIZE as f32) as i64,
        );

        if self.prev_player_chunk != player_chunk_pos {
            println!("player chunk position changed, reset loading iterator");
            self.prev_player_chunk = player_chunk_pos;
            self.chunk_load_iterator = player_chunk_pos.iter_around(self.render_distance);
        }

        for p in self.chunk_load_iterator {
            if self.chunks.get(&p).is_none() {
                let mut new_chunk = Chunk::new(p);
                new_chunk.generate(device);
                self.chunks.insert(p, new_chunk);

                break;
            }
        }

        return true;
    }

    fn get_chunk_to_unload(&mut self) -> Option<Position> {
        // for (pos, _chunk) in self.chunks.iter() {
        //     let chunk_pos = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32) * CHUNK_SIZE as f32;
        //     let dist = (chunk_pos - self.camera.state.eye).length();
        //     if dist > (self.render_distance * CHUNK_SIZE) as f32 * 2. {
        //         println!("chunk at {:?} too far and will be unloaded", pos);
        //         return Some(pos.clone());
        //     }
        // }
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
