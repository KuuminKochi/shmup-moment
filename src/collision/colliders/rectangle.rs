use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::collision::{traits::Collider, components::*};

impl Collider for BoxCollider {
    fn get_type(&self) -> ColliderType {
        ColliderType::Box
    }

    fn collision<T: Collider>(&self, cldr1: &T, tl0: Vec3, tl1: Vec3) -> bool {
        match cldr1.get_type() {
            ColliderType::Box => self.collision_with_box(cldr1.get_shape(), tl0, tl1),
            ColliderType::Circle => self.collision_with_circle(cldr1.get_shape(), tl0, tl1),
            _ => false,
        }
    }

    fn get_shape(&self) -> Vec<f32> {
        vec![self.width, self.height]
    }

    fn collision_with_circle(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool {
        let shp0 = self.get_shape();
        let offset_circle = tl1 - tl0;

        let closest_rect_x_bound = offset_circle[0].clamp(-shp0[0] / 2., shp0[0] / 2.);
        let closest_rect_y_bound = offset_circle[1].clamp(-shp0[1] / 2., shp0[1] / 2.);

        let closest_rect_point = tl0 + Vec3::new(closest_rect_x_bound, closest_rect_y_bound, 0.);

        let dist = tl1.distance(closest_rect_point);

        dist < shp1[0]
    }

    fn collision_with_box(&self, shp1: Vec<f32>, tl0: Vec3, tl1: Vec3) -> bool {
        if let Some(_) = collide(
            tl0,
            Vec2::new(self.width, self.height),
            tl1,
            Vec2::new(shp1[0], shp1[1]),
        ) {
            true
        } else {
            false
        }
    }

    fn get_mask(&self) -> CollisionMask {
        self.collision_mask
    }
}

impl BoxCollider {
    pub fn get_vertex_locations(shp: Vec<f32>, tl: Vec3) -> [Vec3; 4] {
        let half_width = shp[0] / 2.;
        let half_height = shp[1] / 2.;

        [
            Vec3::new(tl.x + half_width, tl.y + half_height, 0.),
            Vec3::new(tl.x - half_width, tl.y + half_height, 0.),
            Vec3::new(tl.x - half_width, tl.y - half_height, 0.),
            Vec3::new(tl.x + half_width, tl.y - half_height, 0.),
        ]
    }
}
