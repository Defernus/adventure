use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

use self::{graphics::GameGraphics, input::GameInput, time::GameTime};

pub mod graphics;
pub mod input;
pub mod time;

pub struct GameSate {
    pub game_input: GameInput,
    pub game_time: GameTime,
    pub game_graphics: GameGraphics,
}

impl GameSate {
    pub(super) async fn new(window: &Window) -> Self {
        Self {
            game_input: GameInput::new(),
            game_time: GameTime::new(),
            game_graphics: GameGraphics::new(window).await,
        }
    }

    pub(super) fn device_input(&mut self, event: &DeviceEvent) -> bool {
        self.game_input.device_input(event)
    }

    pub(super) fn window_input(&mut self, event: &WindowEvent) -> bool {
        self.game_input.window_input(event)
    }

    pub(super) fn pre_update(&mut self, window: &Window) {
        self.game_input.pre_update(window);
        self.game_time.pre_update();
    }

    pub(super) fn post_update(&mut self, window: &Window) {
        self.game_input.post_update();

        if self.game_input.mouse.is_mouse_hidden != self.game_input.mouse.prev_is_cursor_hidden {
            self.game_input.mouse.prev_is_cursor_hidden = self.game_input.mouse.is_mouse_hidden;

            if self.game_input.mouse.is_mouse_hidden {
                let size = window.inner_size();
                window
                    .set_cursor_position(PhysicalPosition::new(size.width / 2, size.height / 2))
                    .expect("failed to set mouse position");
            }
            window.set_cursor_visible(!self.game_input.mouse.is_mouse_hidden);
            window
                .set_cursor_grab(self.game_input.mouse.is_mouse_hidden)
                .expect("failed to grab cursor");
        }
    }

    pub fn hide_cursor(&mut self) {
        self.game_input.mouse.is_mouse_hidden = true;
    }

    pub fn show_cursor(&mut self) {
        self.game_input.mouse.is_mouse_hidden = false;
    }
}
