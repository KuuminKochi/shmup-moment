use bevy::{prelude::KeyCode, utils::HashMap, input::{keyboard::KeyboardInput, ButtonState}};

pub struct KeyStateRes(HashMap<KeyCode, KeyState>);

struct KeyState {
    was_pressed: bool,
    is_pressed: bool,
}

impl Default for KeyState {
    fn default() -> Self {
        KeyState { was_pressed: false, is_pressed: false }
    }
}

impl KeyStateRes {
    pub fn just_pressed(&self, kc: KeyCode) -> bool {
        let state = self.0.get(&kc);
        match state {
            Some(ks) => {
                !ks.was_pressed && ks.is_pressed
            }
            None => {
                println!("Unknown keycode: {:#?}", kc);
                false
            },
        }
    }

    pub fn just_released(&self, kc: KeyCode) -> bool {
        let state = self.0.get(&kc);
        match state {
            Some(ks) => {
                ks.was_pressed && !ks.is_pressed
            }
            None => {
                println!("Unknown keycode: {:#?}", kc);
                false
            },
        }
    }

    pub fn is_pressed(&self, kc: KeyCode) -> bool {
        let state = self.0.get(&kc);
        match state {
            Some(ks) => {
                ks.is_pressed
            }
            None => {
                println!("Unknown keycode: {:#?}", kc);
                false
            },
        }
    }
    
    pub fn update(&mut self, kc: KeyCode, state: ButtonState) {
        match self.0.get_mut(&kc) {
            Some( 
                val
            ) => {
                val.was_pressed = val.is_pressed;
                val.is_pressed = state.is_pressed();
            },
            None => {
                // nice weather huh?
            }
        }
    }
}

impl Default for KeyStateRes {

    fn default() -> Self {
        KeyStateRes(
            HashMap::from([
                (KeyCode::Up, KeyState::default()),
                (KeyCode::Down, KeyState::default()),
                (KeyCode::Left, KeyState::default()),
                (KeyCode::Right, KeyState::default()),
                (KeyCode::Z, KeyState::default()),
                (KeyCode::X, KeyState::default()),
                (KeyCode::C, KeyState::default()),
                (KeyCode::LShift, KeyState::default()),
            ])
        )
    }
}

// impl Key {
//     pub fn just_pressed(&self) -> bool { return !self.was_pressed && self.is_pressed }
//     pub fn just_released(&self) -> bool { return self.was_pressed && !self.is_pressed }
//     pub fn map_keycode(kc: KeyCode) -> Option<Key> {

//     }
// }
