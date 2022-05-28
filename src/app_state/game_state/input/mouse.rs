pub struct MouseInput {
    prev_x: f32,
    prev_y: f32,
    x: f32,
    y: f32,
}

impl MouseInput {
    pub(super) fn new() -> Self {
        Self {
            prev_x: 0.,
            prev_y: 0.,
            x: 0.,
            y: 0.,
        }
    }
    pub(super) fn post_update(&mut self) {
        self.prev_x = self.x;
        self.prev_y = self.y;
    }

    pub(super) fn handle_move(&mut self, x: f32, y: f32) -> bool {
        self.x = x;
        self.y = y;

        return true;
    }

    pub fn get_mouse_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
