use winit::window::Window;

pub struct MouseInput {
    prev_x: f32,
    prev_y: f32,
    x: f32,
    y: f32,
    pub is_mouse_hidden: bool,
    screen_size: (f32, f32),
}

impl MouseInput {
    pub(super) fn new(window: &Window, is_mouse_hidden: bool) -> Self {
        let size = window.inner_size();

        let width = size.width as f32;
        let height = size.height as f32;

        let x = width / 2.;
        let y = height / 2.;

        Self {
            screen_size: (width, height),
            is_mouse_hidden,
            prev_x: x,
            prev_y: y,
            x,
            y,
        }
    }

    pub(super) fn post_update(&mut self) {
        if self.is_mouse_hidden {
            self.x = self.screen_size.0 / 2.;
            self.y = self.screen_size.1 / 2.;
        }
    }

    pub(super) fn handle_move(&mut self, x: f32, y: f32) -> bool {
        match self.is_mouse_hidden {
            false => {
                self.prev_x = self.x;
                self.prev_y = self.y;
            }
            true => {
                self.prev_x = self.screen_size.0 / 2.;
                self.prev_y = self.screen_size.1 / 2.;
            }
        }

        self.x = x;
        self.y = y;

        return true;
    }

    pub fn get_mouse_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_delta(&self) -> (f32, f32) {
        (self.x - self.prev_x, self.y - self.prev_y)
    }
}
