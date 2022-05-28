use std::f32::consts::PI;

use crate::{
    app_state::game_state::{
        input::{InputKey, InputState},
        GameSate,
    },
    vec::Vec3,
};
use wgpu::util::DeviceExt;

use self::{state::CameraState, uniform::CameraUniform};

pub mod state;
pub mod uniform;

const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 10.0;
const FAST_MOVE_FACTOR: f32 = 5.;
const MIN_Y_ANGLE: f32 = PI * 0.1;

pub struct Camera {
    pub state: CameraState,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform: CameraUniform,
    screen_size: (f32, f32),
}

impl Camera {
    pub fn new(device: &wgpu::Device, state: CameraState, screen_size: (f32, f32)) -> Self {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&state);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let result = Self {
            screen_size,
            uniform,
            state,
            buffer,
            bind_group,
            bind_group_layout,
        };

        return result;
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        return &self.bind_group_layout;
    }

    pub fn update_uniform(&mut self, queue: &wgpu::Queue) {
        self.uniform.update_view_proj(&self.state);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    pub fn translate_abs(&mut self, offset: Vec3<f32>) {
        self.state.eye += offset;
        self.state.target += offset;
    }

    pub fn translate(&mut self, offset: Vec3<f32>) {
        let front = self.state.target - self.state.eye;
        let right = front.clone().cross(self.state.up.clone()).normalize();
        let top = right.cross(front);

        let abs_offset = right * offset.x + top * offset.y + front * offset.z;

        self.translate_abs(abs_offset);
    }

    pub fn rotate(&mut self, x: f32, y: f32) {
        let x = x.min(0.1).max(-0.1);
        let mut y = y.min(0.1).max(-0.1);

        let front = (self.state.target - self.state.eye).normalize();
        let front = front.rotate(Vec3::unit_y(), -x);

        let right = front.clone().cross(self.state.up.clone()).normalize();

        let y_angle = front.angle(Vec3::unit_y());
        if y > y_angle - MIN_Y_ANGLE {
            y = y_angle - MIN_Y_ANGLE;
        } else if y < y_angle - PI + MIN_Y_ANGLE {
            y = y_angle - PI + MIN_Y_ANGLE;
        }
        let front = front.rotate(right, y);

        self.state.target = front + self.state.eye;
    }

    pub fn update(&mut self, game_state: &mut GameSate) {
        let dt = game_state.game_time.get_delta_time();
        let mut move_offset = dt * SPEED;
        if game_state.game_input.is_pressed(InputKey::FastMove) {
            move_offset *= FAST_MOVE_FACTOR;
        }

        if game_state.game_input.is_pressed(InputKey::MoveFront) {
            self.translate(Vec3::unit_z() * move_offset);
        }
        if game_state.game_input.is_pressed(InputKey::MoveLeft) {
            self.translate(-Vec3::unit_x() * move_offset);
        }
        if game_state.game_input.is_pressed(InputKey::MoveBack) {
            self.translate(-Vec3::unit_z() * move_offset);
        }
        if game_state.game_input.is_pressed(InputKey::MoveRight) {
            self.translate(Vec3::unit_x() * move_offset);
        }
        if game_state.game_input.is_pressed(InputKey::MoveUp) {
            self.translate_abs(Vec3::unit_y() * move_offset);
        }
        if game_state.game_input.is_pressed(InputKey::MoveDown) {
            self.translate_abs(-Vec3::unit_y() * move_offset);
        }

        match game_state.game_input.get_input_state(InputKey::CursorFree) {
            InputState::JustPressed => {
                game_state.show_cursor();
            }
            InputState::JustReleased => {
                game_state.hide_cursor();
            }
            _ => {}
        }

        let (dx, dy) = game_state.game_input.mouse.get_delta();
        self.rotate(
            dx / self.screen_size.0 * SENSITIVITY,
            -dy / self.screen_size.0 * SENSITIVITY,
        );

        self.uniform.update_view_proj(&self.state);
    }
}
