use bevy::prelude::*;

// GAME CONSTANT
pub const SCREEN_WIDTH: f32 = 640.;
pub const SCREEN_HEIGHT: f32 = 480.;
pub const TIME_STEP: f32 = 1.0 / 60.0;

// PLAYER
pub const PLAYER_COLOUR: Color = Color::rgb(255., 23., 23.);
pub const PLAYER_WIDTH: f32 = 25.;
pub const PLAYER_HEIGHT: f32 = 25.;
pub const PLAYER_SPEED: f32 = 500.;
pub const FOCUS_SCALE: f32 = 2.;

// BULLET
pub const BULLET_COLOUR: Color = Color::rgb(0.0, 0.0, 255.);
pub const PLAYER_BULLET_COOLDOWN: f32 = 0.3;
pub const BULLET_WIDTH: f32 = 20.;
pub const BULLET_HEIGHT: f32 = 20.;
pub const BULLET_SPEED: f32 = 600.;

// ENEMY
pub const ENEMY_COLOUR: Color = Color::rgb(0.0, 255., 255.);
pub const ENEMY_WIDTH: f32 = 2.;
pub const ENEMY_HEIGHT: f32 = 2.;

