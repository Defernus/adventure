use winit::window::Window;

pub struct MouseInput {
    first_delta_event: bool,
    x: f32,
    y: f32,
    delta_x: f32,
    delta_y: f32,
    pub is_mouse_hidden: bool,
}

impl MouseInput {
    pub(super) fn new(is_mouse_hidden: bool) -> Self {
        Self {
            first_delta_event: true,
            is_mouse_hidden,
            delta_x: 0.,
            delta_y: 0.,
            x: 0.,
            y: 0.,
        }
    }

    pub(super) fn pre_update(&mut self, _window: &Window) {}

    pub(super) fn post_update(&mut self) {
        self.delta_x *= 0.5;
        self.delta_y *= 0.5;
    }

    pub(super) fn handle_motion(&mut self, x: f32, y: f32) -> bool {
        if self.first_delta_event {
            self.first_delta_event = false;
            return false;
        }
        self.delta_x = x;
        self.delta_y = y;

        return true;
    }

    pub(super) fn handle_move(&mut self, x: f32, y: f32) -> bool {
        self.x = x;
        self.y = y;

        return true;
    }

    pub fn get_mouse_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_delta(&self) -> (f32, f32) {
        (self.delta_x, self.delta_y)
    }
}
