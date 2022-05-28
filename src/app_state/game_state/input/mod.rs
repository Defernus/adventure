use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Copy, Clone, Debug, EnumCountMacro, EnumIter)]
pub enum InputKey {
    MoveFront,
    MoveLeft,
    MoveBack,
    MoveRight,
    MoveUp,
    MoveDown,
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

    shift_pressed: bool,
    ctrl_pressed: bool,
    // alt_pressed: bool,
    // logo_pressed: bool,
}

impl GameInput {
    pub(super) fn new() -> Self {
        Self {
            input_data: [InputState::Released; InputKey::COUNT],
            shift_pressed: false,
            ctrl_pressed: false,
            // alt_pressed: false,
            // logo_pressed: false,
        }
    }

    fn process_special_keys(&mut self, shift: bool, ctrl: bool, _alt: bool, _logo: bool) -> bool {
        if shift != self.shift_pressed {
            self.shift_pressed = shift;
            return self.handle_key_action(InputKey::MoveUp, shift);
        }
        if ctrl != self.ctrl_pressed {
            self.ctrl_pressed = ctrl;
            return self.handle_key_action(InputKey::MoveDown, ctrl);
        }
        return false;
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
            WindowEvent::ModifiersChanged(v) => {
                self.process_special_keys(v.shift(), v.ctrl(), v.alt(), v.logo())
            }
            _ => false,
        };
        return result;
    }

    pub(super) fn update(&mut self) {
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
