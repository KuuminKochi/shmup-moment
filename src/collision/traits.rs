use crate::collision::components::*;
use bevy::prelude::*;

pub trait Collider {
    fn collision<T: Collider>(&self, cldr1: &T, tl0: Vec3, tl1: Vec3) -> bool;
    fn get_type(&self) -> ColliderType;
    fn get_shape(&self) -> Vec<f32>;
    fn collision_with_circle(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool;
    fn collision_with_box(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool;
    fn get_mask(&self) -> CollisionMask;
}