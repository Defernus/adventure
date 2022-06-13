use std::iter;
use winit::{
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

use crate::world::World;

use self::game_state::GameSate;

pub mod game_state;

pub struct AppState {
    game_state: GameSate,

    world: World,
}

impl AppState {
    pub async fn new(window: &Window) -> Self {
        let game_state = GameSate::new(window).await;
        let world = World::new(window, &game_state);
        Self { game_state, world }
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.game_state.game_graphics.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.game_state.game_graphics.resize(new_size);
        }
    }

    pub fn device_input(&mut self, event: &DeviceEvent) -> bool {
        self.game_state.device_input(event)
    }

    pub fn window_input(&mut self, event: &WindowEvent) -> bool {
        self.game_state.window_input(event)
    }

    pub fn update(&mut self, window: &Window) {
        self.game_state.pre_update(window);

        self.world.update(&mut self.game_state);

        self.game_state.post_update(window);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self
            .game_state
            .game_graphics
            .surface
            .get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.game_state.game_graphics.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.game_state.game_graphics.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            self.world.draw(&mut render_pass);
        }

        self.game_state
            .game_graphics
            .queue
            .submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
