use bevy::prelude::*;
use crate::collision::{*, traits::Collider};

fn test_collision_system<T: Collider + Component, U: Collider + Component>(
    mut commands: Commands,
    query_a: Query<(&T, &Transform)>,
    query_b: Query<(&U, &Transform)>,
) {
    for (cldr0, tf0) in query_a.iter() {
        for (cldr1, tf1) in query_b.iter() {
            if cldr0.collision(cldr1, tf0.translation, tf1.translation) {
                println!("HIT");
            }
        }
    }
}