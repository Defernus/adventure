use std::time::{SystemTime, UNIX_EPOCH};

use winit::event::WindowEvent;

use self::input::GameInput;

pub mod input;

const DEFAULT_DELTA_TIME: u128 = 1000 / 60;

pub struct GameSate {
    start_time: u128,
    delta_time: u128,
    last_time: u128,

    game_input: GameInput,
}

impl GameSate {
    pub fn new() -> Self {
        let start_time = Self::get_current_time();

        let game_input = GameInput::new();
        Self {
            game_input,
            start_time,
            delta_time: DEFAULT_DELTA_TIME,
            last_time: start_time,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.game_input.input(event)
    }

    pub fn pre_update(&mut self) {
        self.update_delta_time();
    }

    pub fn post_update(&mut self) {
        self.game_input.update();
    }

    fn update_delta_time(&mut self) {
        self.delta_time = Self::get_current_time() - self.last_time;
        self.last_time = Self::get_current_time();
    }

    fn get_current_time() -> u128 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        return since_the_epoch.as_millis();
    }

    pub fn get_time_from_start(&self) -> f32 {
        return (Self::get_current_time() - self.start_time) as f32 / 1000.;
    }

    pub fn get_delta_time(&self) -> f32 {
        return self.delta_time as f32 / 1000.;
    }
}
