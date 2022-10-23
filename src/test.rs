use bevy::prelude::*;
use crate::{constant::*, *, collision::{colliders, components::{self, BoxCollider, CollisionMask, CircleCollider}}};

pub fn enemy_spawn(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOUR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100., 0., 0.),
                scale: Vec3::new(ENEMY_WIDTH, ENEMY_HEIGHT, 0.),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(BoxCollider {
            width: ENEMY_WIDTH,
            height: ENEMY_HEIGHT,
            mask: CollisionMask::NPC,
            collision_mask: CollisionMask::PlayerBullet,
        })
        .insert(EnemyStatus {
            is_dead: false,
            is_shoot: false,
        });
}

pub fn player_bullet_spawn(
    mut commands: Commands,
    player_state: Query<(&Transform, &PlayerStatus), With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<BulletTimer>,
) {
    for (player_position, player_status) in player_state.iter() {
        if player_status.is_shoot == true && timer.0.tick(time.delta()).just_finished() {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BULLET_COLOUR,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            player_position.translation.x,
                            player_position.translation.y + 10.0,
                            0.,
                        ),
                        scale: Vec3::new(BULLET_WIDTH, BULLET_HEIGHT, 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(CircleCollider {
                    radius: BULLET_HEIGHT,
                    mask: CollisionMask::PlayerBullet,
                    collision_mask: CollisionMask::NPC,
                })
                .insert(PlayerBullet);

            timer.0.reset();
        }
    }
}

