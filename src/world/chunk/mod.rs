use std::sync::Arc;

use wgpu::{Device, RenderPass};

use crate::{
    app_state::game_state::graphics::{mesh::Mesh, vertex::Vertex},
    utils::{direction::Direction, position::Position, true_mod::true_mod},
    vec::Vec3,
};

use super::{
    generator::Generator,
    voxel::{voxel_data::VoxelData, voxels_to_vertex::append_vertex, Voxel},
};

pub const CHUNK_REAL_SIZE: usize = 16;
pub const CHUNK_VOXELS_SIZE: usize = CHUNK_REAL_SIZE + 1;
pub const CHUNK_VOXELS_VOLUME: usize = CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE;

pub struct Chunk {
    pos: Position,
    mesh: Option<Mesh>,
    voxels: [Voxel; CHUNK_VOXELS_VOLUME],
}

impl Chunk {
    pub fn new(pos: Position) -> Self {
        Self {
            pos: pos,
            voxels: [Voxel {
                value: 0.,
                color: [0.; 3],
            }; CHUNK_VOXELS_VOLUME],
            mesh: None,
        }
    }

    pub fn update_mesh(&mut self, device: &Arc<Device>) {
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

        match &self.mesh {
            Some(mesh) => mesh.destroy(),
            _ => {}
        }

        self.mesh = Some(Mesh::new(vertex, device));
    }

    pub fn generate_voxels(&mut self, generator: &Generator) {
        let offset = Vec3::new(
            (self.pos.x * CHUNK_REAL_SIZE as i64) as f64,
            (self.pos.y * CHUNK_REAL_SIZE as i64) as f64,
            (self.pos.z * CHUNK_REAL_SIZE as i64) as f64,
        );

        generator.generate_voxels(offset, &mut self.voxels, CHUNK_VOXELS_SIZE)
    }

    pub fn generate(&mut self, generator: &Generator, device: &Arc<Device>) {
        self.generate_voxels(generator);
        self.update_mesh(device);
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

    fn cord_to_chunk_cord(v: i64) -> i64 {
        if v < 0 {
            return v / CHUNK_REAL_SIZE as i64 - 1;
        } else {
            return v / CHUNK_REAL_SIZE as i64;
        }
    }

    pub fn get_chunk_pos(pos: Position) -> Position {
        Position::new(
            Self::cord_to_chunk_cord(pos.x),
            Self::cord_to_chunk_cord(pos.y),
            Self::cord_to_chunk_cord(pos.z),
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

    pub fn fill(&mut self, center: Position, radius: f32, voxel: Voxel, value: f32) -> usize {
        let mut count: usize = 0;

        for i in 0..CHUNK_VOXELS_VOLUME {
            let pos = Self::index_to_pos(i) + self.pos.mul_scalar(CHUNK_REAL_SIZE as i64) - center;

            let vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
            let l = vec.length();

            if l < radius && self.voxels[i].value < 0. {
                count += 1;
                self.voxels[i].value = self.voxels[i].value.max(-0.1);
                self.voxels[i].value = self.voxels[i].value + value * ((radius - l) / radius);
                self.voxels[i].value = self.voxels[i].value.min(1.0);

                self.voxels[i].color = voxel.color;
            }
        }

        return count;
    }

    pub fn dig(&mut self, center: Position, radius: f32, value: f32) -> usize {
        let mut count: usize = 0;

        for i in 0..CHUNK_VOXELS_VOLUME {
            let pos = Self::index_to_pos(i) + self.pos.mul_scalar(CHUNK_REAL_SIZE as i64) - center;

            let vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
            let l = vec.length();

            if l < radius && self.voxels[i].value >= 0. {
                count += 1;
                self.voxels[i].value -= value * (radius - l) / radius;
                self.voxels[i].value = self.voxels[i].value.max(-0.1);
            }
        }

        return count;
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
        match self.mesh.as_ref() {
            Some(mesh) => mesh.draw(render_pass),
            _ => {}
        }
    }

    pub fn get_position(&self) -> Position {
        return self.pos;
    }
}
