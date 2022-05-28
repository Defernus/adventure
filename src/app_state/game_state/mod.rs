use winit::event::WindowEvent;

use self::{input::GameInput, time::GameTime};

pub mod input;
pub mod time;

pub struct GameSate {
    pub game_input: GameInput,
    pub game_time: GameTime,
}

impl GameSate {
    pub(super) fn new() -> Self {
        Self {
            game_input: GameInput::new(),
            game_time: GameTime::new(),
        }
    }

    pub(super) fn input(&mut self, event: &WindowEvent) -> bool {
        self.game_input.input(event)
    }

    pub(super) fn pre_update(&mut self) {
        self.game_time.update();
    }

    pub(super) fn post_update(&mut self) {
        self.game_input.update();
    }
}
