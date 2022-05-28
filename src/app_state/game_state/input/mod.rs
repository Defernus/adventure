use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

use self::mouse::MouseInput;

pub mod mouse;

#[derive(Copy, Clone, Debug, EnumCountMacro, EnumIter)]
pub enum InputKey {
    MoveFront,
    MoveLeft,
    MoveBack,
    MoveRight,
    MoveUp,
    MoveDown,
    CursorFree,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputState {
    Released,
    Pressed,
    JustPressed,
    JustReleased,
}

#[allow(dead_code)]
pub struct GameInput {
    input_data: [InputState; InputKey::COUNT],

    pub mouse: MouseInput,

    is_in_focus: bool,
}

impl GameInput {
    pub(super) fn new(window: &Window) -> Self {
        Self {
            input_data: [InputState::Released; InputKey::COUNT],
            mouse: MouseInput::new(window, true),
            is_in_focus: true,
        }
    }

    pub(super) fn input(&mut self, event: &WindowEvent) -> bool {
        let result = match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => self.keys_action(key, state == &ElementState::Pressed),
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse.handle_move(position.x as f32, position.y as f32)
            }
            WindowEvent::CursorLeft { .. } => {
                self.is_in_focus = false;
                return true;
            }
            WindowEvent::CursorEntered { .. } => {
                self.is_in_focus = true;
                return true;
            }
            _ => false,
        };
        return result;
    }

    pub(super) fn post_update(&mut self) {
        self.mouse.post_update();
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

    fn handle_key_action(&mut self, key: InputKey, press: bool) -> bool {
        match press {
            true => self.press_key(key),
            false => self.release_key(key),
        }
    }

    fn keys_action(&mut self, key: &VirtualKeyCode, pressed: bool) -> bool {
        match key {
            VirtualKeyCode::W => self.handle_key_action(InputKey::MoveFront, pressed),
            VirtualKeyCode::A => self.handle_key_action(InputKey::MoveLeft, pressed),
            VirtualKeyCode::S => self.handle_key_action(InputKey::MoveBack, pressed),
            VirtualKeyCode::D => self.handle_key_action(InputKey::MoveRight, pressed),
            VirtualKeyCode::LShift => self.handle_key_action(InputKey::MoveDown, pressed),
            VirtualKeyCode::Space => self.handle_key_action(InputKey::MoveUp, pressed),
            VirtualKeyCode::Q => self.handle_key_action(InputKey::CursorFree, pressed),
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

    pub fn is_window_in_focus(&self) -> bool {
        return self.is_in_focus;
    }
}
