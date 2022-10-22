use crate::collision::{components::*, traits::Collider};
use bevy::{prelude::*, sprite::collide_aabb::collide};

impl Collider for CircleCollider {
    fn get_type(&self) -> ColliderType {
        ColliderType::Circle
    }

    fn collision<T: Collider>(&self, cldr1: &T, tl0: Vec3, tl1: Vec3) -> bool {
        if self.get_collision_mask() != cldr1.get_mask() {
            return false;
        }
        match cldr1.get_type() {
            ColliderType::Box => self.collision_with_box(cldr1.get_shape(), tl0, tl1),
            ColliderType::Circle => self.collision_with_circle(cldr1.get_shape(), tl0, tl1),
            _ => false,
        }
    }

    fn get_shape(&self) -> Vec<f32> {
        vec![self.radius]
    }

    fn collision_with_circle(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool {
        let shifted_tl = tl1 - tl0;
        let dist = (shifted_tl.x.powf(2.) + shifted_tl.y.powf(2.)).sqrt();

        dist < self.get_shape()[0] + shp1[0]
    }

    fn collision_with_box(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool {
        let offset_circle = tl0 - tl1;

        let closest_rect_x_bound = offset_circle[0].clamp(-shp1[0] / 2., shp1[0] / 2.);
        let closest_rect_y_bound = offset_circle[1].clamp(-shp1[1] / 2., shp1[1] / 2.);

        let closest_rect_point = tl1 + Vec3::new(closest_rect_x_bound, closest_rect_y_bound, 0.);

        let dist = tl0.distance(closest_rect_point);

        dist < self.radius
    }

    fn get_mask(&self) -> CollisionMask {
        self.mask
    }

    fn get_collision_mask(&self) -> CollisionMask {
        self.collision_mask
    }
}
