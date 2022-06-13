use std::{
    collections::{self, BTreeMap},
    sync::Arc,
    thread::{self, JoinHandle},
};

use wgpu::{include_wgsl, Device, RenderPass, RenderPipeline, SurfaceConfiguration};
use winit::window::Window;

use crate::{
    app_state::game_state::{input::InputKey, GameSate},
    player::Player,
    sun::Sun,
    texture,
    utils::position::{Position, PositionAroundIterator},
    vec::Vec3,
    vertex::Vertex,
};

use self::{
    chunk::{Chunk, CHUNK_REAL_SIZE, CHUNK_VOXELS_SIZE},
    generator::Generator,
    voxel::Voxel,
};

pub mod chunk;
pub mod generator;
pub mod voxel;

pub struct World {
    chunks: collections::BTreeMap<Position, Chunk>,
    render_pipeline: RenderPipeline,
    pub player: Player,

    chunk_generating_per_frame: usize,
    render_distance: usize,
    prev_player_chunk: Position,
    chunk_load_iterator: PositionAroundIterator,

    generation_enabled: bool,
    generator: Arc<Generator>,

    sun: Sun,
}

impl World {
    pub fn new(window: &Window, device: &Arc<Device>, config: &SurfaceConfiguration) -> Self {
        let screen_size = window.inner_size();
        let shader = device.create_shader_module(&include_wgsl!("shaders/main.wgsl"));

        let sun = Sun::new(&device);

        let player = Player::new(
            device,
            (screen_size.width as f32, screen_size.height as f32),
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[player.get_bind_group_layout(), sun.get_bind_group_layout()],
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

        let chunks = BTreeMap::new();

        let render_distance = 8;

        return World {
            sun,
            chunks,
            chunk_generating_per_frame: 4,
            render_distance,
            render_pipeline,
            player,
            generation_enabled: true,
            prev_player_chunk: Position::new(0, 0, 0),
            chunk_load_iterator: Position::new(0, 0, 0).iter_around(render_distance),
            generator: Arc::new(Generator::new()),
        };
    }

    fn get_max_chunk(&self) -> usize {
        (self.render_distance * 2 + 1).pow(3)
    }

    fn load_chunk(&mut self, device: &Arc<Device>, _game_state: &mut GameSate) -> bool {
        if self.chunks.len() >= self.get_max_chunk() {
            return false;
        }

        let camera_pos = self.player.get_pos();
        let player_chunk_pos = Position::new(
            (camera_pos.x / CHUNK_VOXELS_SIZE as f32) as i64,
            (camera_pos.y / CHUNK_VOXELS_SIZE as f32) as i64,
            (camera_pos.z / CHUNK_VOXELS_SIZE as f32) as i64,
        );

        if self.prev_player_chunk != player_chunk_pos {
            self.prev_player_chunk = player_chunk_pos;
            self.chunk_load_iterator = player_chunk_pos.iter_around(self.render_distance);
        }

        let max_chunks = self.chunk_generating_per_frame;
        let mut chunk_generated: Vec<Position> = vec![];

        let mut handles: Vec<JoinHandle<Chunk>> = vec![];

        for p in self.chunk_load_iterator {
            if self.chunks.get(&p).is_none() {
                let np = p.clone();
                let gen = self.generator.clone();
                let device = device.clone();

                handles.push(thread::spawn(move || {
                    let mut new_chunk = Chunk::new(np);
                    new_chunk.generate(&gen, &device);
                    return new_chunk;
                }));

                chunk_generated.push(p.clone());
                if chunk_generated.len() >= max_chunks {
                    break;
                }
            }
        }

        for h in handles {
            let chunk = h.join().unwrap();
            self.chunks.insert(chunk.get_position(), chunk);
        }

        return chunk_generated.len() > 0;
    }

    fn get_chunk_to_unload(&mut self) -> Option<Position> {
        for (chunk_pos, _chunk) in self.chunks.iter() {
            let player_pos = self.player.get_chunk_pos();
            let delta = player_pos - chunk_pos.clone();

            if delta.x.abs().max(delta.y.abs()).max(delta.z.abs()) > self.render_distance as i64 + 1
            {
                // println!("chunk at {:?} too far and will be unloaded", chunk_pos);
                return Some(chunk_pos.clone());
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

    pub fn cast_ray(
        &self,
        from: Vec3<f32>,
        dir: Vec3<f32>,
        max_dist: f32,
    ) -> Option<(Position, Voxel)> {
        let step_size: f32 = 0.01;
        let mut ray_len: f32 = 0.;

        let mut chunk = self.get_chunk_by_vec(from)?;
        println!("start {:?} {:?} {:?}", from, dir, chunk.get_position());

        while ray_len < max_dist {
            let pos = from + dir.mul_scalar(ray_len);

            let in_chunk_pos =
                Chunk::get_in_chunk_pos(Position::new(pos.x as i64, pos.y as i64, pos.z as i64));
            let voxel = chunk.get_voxel(in_chunk_pos)?;

            if voxel.value > 0. {
                println!("{} {:?} {:?}", ray_len, in_chunk_pos, pos);
                let result_pos =
                    in_chunk_pos + chunk.get_position().mul_scalar(CHUNK_REAL_SIZE as i64);
                return Some((result_pos, voxel));
            }

            let new_chunk_pos = Self::get_chunk_cord_by_vec(pos);

            if new_chunk_pos != chunk.get_position() {
                chunk = self.get_chunk(new_chunk_pos)?;
            };

            ray_len += step_size;
        }

        None
    }

    pub fn mine(&mut self, device: &Arc<Device>, game_state: &mut GameSate) -> Option<()> {
        let dt = game_state.game_time.get_delta_time();
        let pos = self.player.get_pos();
        let dir = self.player.get_look_dir();

        let pos = self.cast_ray(pos, dir, 32.)?.0;

        let chunks_to_dig = Chunk::get_chunk_pos(pos).iter_neighbors(true);
        for chunk_pos in chunks_to_dig {
            let chunk = match self.get_chunk_mut(chunk_pos) {
                Some(chunk) => chunk,
                _ => {
                    let mut chunk = Chunk::new(chunk_pos.clone());
                    chunk.generate(&self.generator, device);
                    self.chunks.insert(chunk_pos.clone(), chunk);
                    self.chunks.get_mut(&chunk_pos)?
                }
            };
            let modified_voxels = chunk.dig(pos, 6., dt / 20.);
            if modified_voxels > 0 {
                chunk.update_mesh(device);
            }
        }
        return Some(());
    }

    pub fn fill(&mut self, device: &Arc<Device>, game_state: &mut GameSate) -> Option<()> {
        let dt = game_state.game_time.get_delta_time();
        let pos = self.player.get_pos();
        let dir = self.player.get_look_dir();

        let (pos, voxel) = self.cast_ray(pos, dir, 32.)?;

        let chunks_to_dig = Chunk::get_chunk_pos(pos).iter_neighbors(true);
        for chunk_pos in chunks_to_dig {
            let chunk = match self.get_chunk_mut(chunk_pos) {
                Some(chunk) => chunk,
                _ => {
                    let mut chunk = Chunk::new(chunk_pos.clone());
                    chunk.generate(&self.generator, device);
                    self.chunks.insert(chunk_pos.clone(), chunk);
                    self.chunks.get_mut(&chunk_pos)?
                }
            };
            let modified_voxels = chunk.fill(pos, 6., voxel, dt / 20.);
            if modified_voxels > 0 {
                chunk.update_mesh(device);
            }
        }
        return Some(());
    }

    pub fn process_input(&mut self, device: &Arc<Device>, game_state: &mut GameSate) {
        if game_state.game_input.is_pressed(InputKey::Mine) {
            self.mine(device, game_state);
        } else if game_state.game_input.is_pressed(InputKey::Fill) {
            self.fill(device, game_state);
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, device: &Arc<Device>, game_state: &mut GameSate) {
        self.player.update(game_state);

        if self.generation_enabled {
            self.load_chunk(device, game_state);
        }

        self.process_input(device, game_state);

        self.chunks.iter_mut().for_each(|(_pos, chunk)| {
            chunk.update();
        });

        if game_state
            .game_input
            .is_just_pressed(InputKey::ChunkGeneration)
        {
            self.generation_enabled = !self.generation_enabled;
        }

        self.player.update_uniform(queue);
        self.sun.update_uniform(queue);

        if self.generation_enabled {
            self.unload_chunk(game_state);
        }
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        self.player.draw(render_pass);
        render_pass.set_bind_group(1, self.sun.get_bind_group(), &[]);

        for (_pos, chunk) in self.chunks.iter() {
            chunk.draw(render_pass);
        }
    }

    pub fn get_chunk_cord_by_vec(vec: Vec3<f32>) -> Position {
        let x = vec.x / CHUNK_REAL_SIZE as f32;
        let y = vec.y / CHUNK_REAL_SIZE as f32;
        let z = vec.z / CHUNK_REAL_SIZE as f32;
        Position::new(
            if x < 0. { x as i64 - 1 } else { x as i64 },
            if y < 0. { y as i64 - 1 } else { y as i64 },
            if z < 0. { z as i64 - 1 } else { z as i64 },
        )
    }

    pub fn get_chunk_by_vec(&self, vec: Vec3<f32>) -> Option<&Chunk> {
        self.chunks.get(&Self::get_chunk_cord_by_vec(vec))
    }

    fn get_chunk_mut(&mut self, chunk_pos: Position) -> Option<&mut Chunk> {
        self.chunks.get_mut(&chunk_pos)
    }

    pub fn get_chunk(&self, chunk_pos: Position) -> Option<&Chunk> {
        self.chunks.get(&chunk_pos)
    }

    pub fn get_voxel(&self, pos: Position) -> Option<Voxel> {
        let chunk_pos = Chunk::get_chunk_pos(pos);

        match self.get_chunk(chunk_pos) {
            Some(chunk) => {
                let in_chunk_pos = Chunk::get_in_chunk_pos(pos);
                return chunk.get_voxel(in_chunk_pos);
            }
            None => {
                return None;
            }
        };
    }
}
