use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_DELTA_TIME: u128 = 1000 / 60;

pub struct GameTime {
    start_time: u128,
    delta_time: u128,
    last_time: u128,
}

impl GameTime {
    pub(super) fn new() -> Self {
        let start_time = Self::get_current_time();

        Self {
            start_time,
            delta_time: DEFAULT_DELTA_TIME,
            last_time: start_time,
        }
    }
    pub(super) fn pre_update(&mut self) {
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
