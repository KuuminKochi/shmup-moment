use bevy::prelude::*;

// COMPONENTS
#[derive(Component)]
pub enum WorldChapters {
    One,
    Two,
    Three,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStatus {
    is_dead: bool,
    is_shoot: bool,
    is_focus: bool,
}

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Component)]
pub struct BulletTimer(Timer);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyStatus {
    is_dead: bool,
    is_shoot: bool,
}




#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

