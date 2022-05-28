use winit::{dpi::LogicalPosition, event::WindowEvent, window::Window};

use self::{input::GameInput, time::GameTime};

pub mod input;
pub mod time;

pub struct GameSate {
    pub game_input: GameInput,
    pub game_time: GameTime,
}

impl GameSate {
    pub(super) fn new(window: &Window) -> Self {
        Self {
            game_input: GameInput::new(window),
            game_time: GameTime::new(),
        }
    }

    pub(super) fn input(&mut self, event: &WindowEvent) -> bool {
        self.game_input.input(event)
    }

    pub(super) fn pre_update(&mut self) {
        self.game_time.pre_update();
    }

    pub(super) fn post_update(&mut self, window: &Window) {
        self.game_input.post_update();

        let window_size = window.inner_size();

        if self.game_input.mouse.is_mouse_hidden {
            let result = window.set_cursor_position(LogicalPosition::new(
                window_size.width / 4,
                window_size.height / 4,
            ));
            match result {
                Err(e) => panic!("failed to set cursor position {}", e),
                _ => {}
            }
        }
        window.set_cursor_visible(!self.game_input.mouse.is_mouse_hidden);
    }

    pub fn hide_cursor(&mut self) {
        self.game_input.mouse.is_mouse_hidden = true;
    }

    pub fn show_cursor(&mut self) {
        self.game_input.mouse.is_mouse_hidden = false;
    }
}
