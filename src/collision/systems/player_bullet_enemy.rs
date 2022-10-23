//fn check_for_collisions(
//    mut commands: Commands,
//    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
//    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
//) {
//    for (bullet_entity, player_bullet_transform) in player_bullet_query.iter() {
//        for (enemy_entity, enemy_transform) in enemy_query.iter() {
//            let collision = collide(
//                player_bullet_transform.translation,
//                player_bullet_transform.scale.truncate(),
//                enemy_transform.translation,
//                enemy_transform.scale.truncate(),
//            );
//
//            if let Some(collision) = collision {
//                match collision {
//                    Collision::Left => {}
//                    Collision::Right => {}
//                    Collision::Top => {}
//                    Collision::Bottom => {}
//                    Collision::Inside => {
//                        commands.entity(enemy_entity).despawn();
//                        commands.entity(bullet_entity).despawn();
//                    }
//                };
//            }
//        }
//    }
//}
//
use bevy::{prelude::*};

use crate::collision::{traits::Collider, *};
use crate::{collision::components::{CircleCollider, BoxCollider}, PlayerBullet, Enemy};

// player_bullet_orientated
pub fn player_bullet_enemy(
    mut commands: Commands,
    player_bullet_query: Query<(&CircleCollider, &Transform, Entity), With<PlayerBullet>>,
    enemy_query: Query<(&BoxCollider, &Transform, Entity), With<Enemy>>,
    ) {
    for (cldr0, tf0, e0) in player_bullet_query.iter() {
        for (cldr1, tf1, e1) in enemy_query.iter() {
            if cldr0.collision(cldr1, tf0.translation, tf1.translation) {
                commands
                    .entity(e0)
                    .despawn();
            }
        }
    }
}

pub fn enemy_player_bullet(
    mut commands: Commands,
    enemy_query: Query<(&BoxCollider, &Transform, Entity), With<Enemy>>,
    player_bullet_query: Query<(&CircleCollider, &Transform, Entity), With<PlayerBullet>>,
    ) {
    for (cldr0, tf0, e0) in enemy_query.iter() {
        for (cldr1, tf1, e1) in player_bullet_query.iter() {
            if cldr0.collision(cldr1, tf0.translation, tf1.translation) {
                 commands
                     .entity(e0)
                     .despawn();
            }
        }
    }
}














