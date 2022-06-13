use crate::{
    app_state::game_state::GameSate, utils::position::Position, vec::Vec3,
    world::chunk::CHUNK_REAL_SIZE,
};
use wgpu::{BindGroupLayout, Device, Queue, RenderPass};

use self::camera::{state::CameraState, Camera};

mod camera;

pub struct Player {
    cam: Camera,
}

impl Player {
    pub fn new(device: &Device, screen_size: (f32, f32)) -> Self {
        let state = CameraState {
            eye: (0., 0., 0.).into(),
            target: (0., 0., -1.0).into(),
            up: Vec3::new(0., 1., 0.),
            aspect: screen_size.0 / screen_size.1,
            fov_y: 70.0,
            z_near: 0.1,
            z_far: 1024.0,
        };
        Self {
            cam: Camera::new(device, state, (screen_size.0, screen_size.1)),
        }
    }

    pub fn get_chunk_pos(&self) -> Position {
        Position::new(
            (self.cam.state.eye.x / CHUNK_REAL_SIZE as f32) as i64,
            (self.cam.state.eye.y / CHUNK_REAL_SIZE as f32) as i64,
            (self.cam.state.eye.z / CHUNK_REAL_SIZE as f32) as i64,
        )
    }

    pub fn get_pos(&self) -> Vec3<f32> {
        self.cam.state.eye
    }

    pub fn draw<'a>(self: &'a Self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_bind_group(0, self.cam.get_bind_group(), &[]);
    }

    pub fn update_uniform(&mut self, queue: &Queue) {
        self.cam.update_uniform(queue);
    }

    pub fn update(&mut self, game_state: &mut GameSate) {
        self.cam.update(game_state);
    }

    pub fn get_bind_group_layout(&self) -> &BindGroupLayout {
        self.cam.get_bind_group_layout()
    }
}
