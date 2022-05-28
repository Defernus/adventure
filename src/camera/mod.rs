use crate::app_state::game_state::{input::InputKey, GameSate};
use wgpu::util::DeviceExt;

use self::{state::CameraState, uniform::CameraUniform};

pub mod state;
pub mod uniform;

pub struct Camera {
    pub state: CameraState,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform: CameraUniform,
}

impl Camera {
    pub fn new(device: &wgpu::Device, state: CameraState) -> Self {
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

    pub fn translate(&mut self, offset: cgmath::Point3<f32>) {
        self.state.eye = cgmath::Point3::new(
            self.state.eye.x + offset.x,
            self.state.eye.y + offset.y,
            self.state.eye.z + offset.z,
        );
    }

    pub fn update(&mut self, game_state: &GameSate) {
        if game_state.game_input.is_pressed(InputKey::MoveFront) {
            self.translate((0.0, 0.0, 1.0 * game_state.game_time.get_delta_time()).into());
            self.uniform.update_view_proj(&self.state);
        }
    }
}
