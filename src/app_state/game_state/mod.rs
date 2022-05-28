use winit::{dpi::LogicalPosition, event::WindowEvent, window::Window};

use self::{input::GameInput, time::GameTime};

pub mod input;
pub mod time;

const DEFAULT_CURSOR_HIDE_SATE: bool = true;

pub struct GameSate {
    pub game_input: GameInput,
    pub game_time: GameTime,

    hide_cursor: bool,
}

impl GameSate {
    pub(super) fn new() -> Self {
        Self {
            game_input: GameInput::new(),
            game_time: GameTime::new(),
            hide_cursor: DEFAULT_CURSOR_HIDE_SATE,
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

        if self.hide_cursor {
            window.set_cursor_position(LogicalPosition::new(
                window_size.width / 4,
                window_size.height / 4,
            ));
        }
        window.set_cursor_visible(!self.hide_cursor);
    }

    pub fn hide_cursor(&mut self) {
        self.hide_cursor = true;
    }

    pub fn show_cursor(&mut self) {
        self.hide_cursor = false;
    }
}
