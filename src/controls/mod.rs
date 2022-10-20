use bevy::prelude::*;

pub mod resources;
pub mod systems;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {}
}
