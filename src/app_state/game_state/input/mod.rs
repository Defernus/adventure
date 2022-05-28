use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Copy, Clone, Debug, EnumCountMacro, EnumIter)]
pub enum InputKey {
    MoveFront,
    MoveLeft,
    MoveBack,
    MoveRight,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputState {
    Released,
    Pressed,
    JustPressed,
    JustReleased,
}

pub struct GameInput {
    input_data: [InputState; InputKey::COUNT],
}

impl GameInput {
    pub fn new() -> Self {
        Self {
            input_data: [InputState::Released; InputKey::COUNT],
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        let result = match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => self.key_pressed(key),
                ElementState::Released => self.key_released(key),
            },
            _ => false,
        };
        return result;
    }

    pub fn update(&mut self) {
        for i in 0..InputKey::COUNT {
            if self.input_data[i] == InputState::JustPressed {
                self.input_data[i] = InputState::Pressed;
            } else if self.input_data[i] == InputState::JustReleased {
                self.input_data[i] = InputState::Released;
            }
        }
    }

    fn press_key(&mut self, key: InputKey) -> bool {
        match self.input_data[key as usize] {
            InputState::JustPressed => {
                self.input_data[key as usize] = InputState::Pressed;
                return true;
            }
            InputState::Released | InputState::JustReleased => {
                self.input_data[key as usize] = InputState::JustPressed;
                return true;
            }
            InputState::Pressed => {
                return false;
            }
        }
    }

    fn release_key(&mut self, key: InputKey) -> bool {
        match self.input_data[key as usize] {
            InputState::JustReleased => {
                self.input_data[key as usize] = InputState::Released;
                return true;
            }
            InputState::JustPressed | InputState::Pressed => {
                self.input_data[key as usize] = InputState::JustReleased;
                return true;
            }
            InputState::Released => {
                return false;
            }
        }
    }

    fn key_pressed(&mut self, key: &VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::W => self.press_key(InputKey::MoveFront),
            VirtualKeyCode::A => self.press_key(InputKey::MoveLeft),
            VirtualKeyCode::S => self.press_key(InputKey::MoveBack),
            VirtualKeyCode::D => self.press_key(InputKey::MoveRight),
            _ => false,
        }
    }

    fn key_released(&mut self, key: &VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::W => self.release_key(InputKey::MoveFront),
            VirtualKeyCode::A => self.release_key(InputKey::MoveLeft),
            VirtualKeyCode::S => self.release_key(InputKey::MoveBack),
            VirtualKeyCode::D => self.release_key(InputKey::MoveRight),
            _ => false,
        }
    }

    pub fn get_input_state(&self, input: InputKey) -> InputState {
        return self.input_data[input as usize];
    }

    pub fn is_pressed(&self, input: InputKey) -> bool {
        match self.get_input_state(input) {
            InputState::Pressed | InputState::JustPressed => true,
            _ => false,
        }
    }
}
