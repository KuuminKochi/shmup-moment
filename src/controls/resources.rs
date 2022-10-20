use bevy::prelude::KeyCode;

pub struct KeyStateRes {
    up: Key,
    down: Key,
    left: Key,
    right: Key,
    z: Key,
    x: Key,
    c: Key,
    shift: Key,
    esc: Key,
}

pub struct Key {
    kc: KeyCode,
    was_pressed: bool,
    is_pressed: bool,
}

impl Key {
    pub fn just_pressed(&self) -> bool { return !self.was_pressed && self.is_pressed }
}
