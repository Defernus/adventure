use wgpu::util::DeviceExt;

use crate::vec::Vec3;

pub struct Sun {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    uniform: SunUniform,
}

impl Sun {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = SunUniform::new();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sun Buffer"),
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
            label: Some("sun_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("sun_bind_group"),
        });

        let result = Self {
            uniform,
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
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SunUniform {
    pub direction: [f32; 4],
}

impl SunUniform {
    pub fn new() -> Self {
        let vec = Vec3::new(-1., 7., -4.).normalize();
        Self {
            direction: [vec.x, vec.y, vec.z, 0.0],
        }
    }
}
