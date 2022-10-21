use bevy::prelude::*;

#[derive(Debug)]
pub enum ColliderType {
    Box,
    Circle,
}

#[derive(Component, Debug)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
    pub mask: CollisionMask,
    pub collision_mask: CollisionMask,
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
    pub mask: CollisionMask,
    pub collision_mask: CollisionMask,
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionMask {
    Player,
    PlayerBullet,
    NPC,
    Bullet,
}