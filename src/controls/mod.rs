use bevy::prelude::*;

use self::resources::KeyStateRes;

pub mod resources;
pub mod systems;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {

    }
}