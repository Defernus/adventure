use noise::NoiseFn;
use wgpu::{util::DeviceExt, Device, RenderPass};

use crate::{
    utils::{direction::Direction, position::Position, true_mod::true_mod},
    vertex::Vertex,
};

use super::{
    voxel::{voxel_data::VoxelData, voxels_to_vertex::append_vertex, Voxel},
    World,
};

pub const CHUNK_REAL_SIZE: usize = 16;
pub const CHUNK_VOXELS_SIZE: usize = CHUNK_REAL_SIZE + 1;
pub const CHUNK_VOXELS_VOLUME: usize = CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE;

pub struct Chunk {
    pos: Position,
    world: *mut World,
    vertex_data: Option<ChunkVertex>,
    voxels: [Voxel; CHUNK_VOXELS_VOLUME],

    noise_scale: f64,
    noise_threshold: f32,
    simplex: noise::OpenSimplex,
}

struct ChunkVertex {
    vertex: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
}

impl Chunk {
    pub fn new(world: *mut World, pos: Position) -> Self {
        Self {
            world,
            pos: pos,
            voxels: [Voxel { value: 1., id: 0 }; CHUNK_VOXELS_VOLUME],
            vertex_data: None,
            noise_scale: 0.03,
            noise_threshold: 0.4,
            simplex: noise::OpenSimplex::new(),
        }
    }

    pub fn generate_vertex(&mut self) -> Vec<Vertex> {
        let mut vertex: Vec<Vertex> = Vec::new();
        for x in 0..CHUNK_REAL_SIZE {
            for y in 0..CHUNK_REAL_SIZE {
                for z in 0..CHUNK_REAL_SIZE {
                    append_vertex(
                        Position::new(x as i64, y as i64, z as i64),
                        self,
                        &mut vertex,
                    );
                }
            }
        }

        for v in vertex.iter_mut() {
            v.position = [
                v.position[0] + (self.pos.x * CHUNK_REAL_SIZE as i64) as f32,
                v.position[1] + (self.pos.y * CHUNK_REAL_SIZE as i64) as f32,
                v.position[2] + (self.pos.z * CHUNK_REAL_SIZE as i64) as f32,
            ]
        }

        return vertex;
    }

    fn generate_block(&mut self, index: usize) {
        let pos = Self::index_to_pos(index);

        let x = (self.pos.x * CHUNK_REAL_SIZE as i64 + pos.x) as f64;
        let y = (self.pos.y * CHUNK_REAL_SIZE as i64 + pos.y) as f64;
        let z = (self.pos.z * CHUNK_REAL_SIZE as i64 + pos.z) as f64;

        let mut noise_v = self.simplex.get([
            x * self.noise_scale,
            y * self.noise_scale,
            z * self.noise_scale,
        ]) as f32;
        noise_v += 1.0;
        noise_v /= 2.0;

        noise_v *= 1.0 - (y as f32 / CHUNK_REAL_SIZE as f32).min(1.0).max(0.0);
        noise_v -= self.noise_threshold;
        noise_v /= self.noise_scale as f32;

        self.voxels[index].value = noise_v / 2.;
        if noise_v < 0. {
            self.voxels[index].id = 0;
        } else {
            self.voxels[index].id = 1;
        };
    }

    pub fn generate(&mut self, device: &Device) {
        // println!("generating chunk {:?}", self.pos);

        for i in 0..CHUNK_VOXELS_VOLUME {
            let val = self.generate_block(i);
        }

        let vertex = self.generate_vertex();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_data = Some(ChunkVertex {
            vertex,
            vertex_buffer,
        });
    }

    pub fn pos_to_index(pos: Position) -> Option<usize> {
        if !Self::check_pos_in_chunk(pos) {
            return None;
        }
        return Some(
            pos.x as usize
                + pos.y as usize * CHUNK_VOXELS_SIZE
                + pos.z as usize * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE,
        );
    }

    pub fn get_chunk_pos(pos: Position) -> Position {
        Position::new(
            pos.x / CHUNK_REAL_SIZE as i64,
            pos.y / CHUNK_REAL_SIZE as i64,
            pos.z / CHUNK_REAL_SIZE as i64,
        )
    }
    pub fn get_in_chunk_pos(pos: Position) -> Position {
        Position::new(
            true_mod(pos.x, CHUNK_REAL_SIZE as i64),
            true_mod(pos.y, CHUNK_REAL_SIZE as i64),
            true_mod(pos.z, CHUNK_REAL_SIZE as i64),
        )
    }

    pub fn index_to_pos(index: usize) -> Position {
        return Position {
            x: (index % CHUNK_VOXELS_SIZE) as i64,
            y: ((index / CHUNK_VOXELS_SIZE) % CHUNK_VOXELS_SIZE) as i64,
            z: (index / CHUNK_VOXELS_SIZE / CHUNK_VOXELS_SIZE) as i64,
        };
    }

    pub fn pos_to_relative(&self, pos: Position) -> Position {
        return Position::new(
            pos.x - self.pos.x * CHUNK_VOXELS_SIZE as i64,
            pos.y - self.pos.y * CHUNK_VOXELS_SIZE as i64,
            pos.y - self.pos.y * CHUNK_VOXELS_SIZE as i64,
        );
    }

    pub fn set_voxel(&mut self, in_chunk_position: Position, voxel: Voxel) -> bool {
        match Self::pos_to_index(in_chunk_position) {
            Some(index) => {
                self.voxels[index] = voxel;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn get_voxel(&self, in_chunk_position: Position) -> Option<Voxel> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                return Some(self.voxels[index]);
            }
            _ => None,
        }
    }

    pub fn get_voxel_data(&self, in_chunk_position: Position) -> Option<VoxelData> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                let voxel = self.voxels[index];
                return Some(VoxelData {
                    voxel,
                    chunk: self,
                    in_chunk_position,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn set_pos(&self, in_chunk_position: Position) -> Option<VoxelData> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                let voxel = self.voxels[index];
                return Some(VoxelData {
                    voxel,
                    chunk: self,
                    in_chunk_position,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn check_pos_in_chunk(pos: Position) -> bool {
        return pos.x >= 0
            && pos.x < CHUNK_VOXELS_SIZE as i64
            && pos.y >= 0
            && pos.y < CHUNK_VOXELS_SIZE as i64
            && pos.z >= 0
            && pos.z < CHUNK_VOXELS_SIZE as i64;
    }

    pub fn get_neighbor(&self, pos: Position, dir: Direction) -> Option<VoxelData> {
        let result_pos = pos.get_neighbor(dir);

        match Self::pos_to_index(result_pos.clone()) {
            Some(index) => {
                return Some(VoxelData {
                    voxel: self.voxels[index],
                    in_chunk_position: result_pos.clone(),
                    chunk: &self,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn update(&self) {}

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        match self.vertex_data.as_ref() {
            Some(vertex_data) => {
                render_pass.set_vertex_buffer(0, vertex_data.vertex_buffer.slice(..));
                render_pass.draw(0..vertex_data.vertex.len() as u32, 0..1);
            }
            _ => {}
        }
    }

    pub fn get_position(&self) -> Position {
        return self.pos;
    }
}
