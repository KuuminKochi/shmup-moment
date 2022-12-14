use std::f32::consts::PI;
use collision::CollisionPlugin;
use constant::*;
use test::*;
// use external library
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// mods
mod collision;
mod controls;
pub mod constant;
pub mod test;

// COMPONENTS

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
pub struct EnemyBullet;

#[derive(Default)]
pub struct CollisionEvent;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup)
        .add_startup_system(access_window_system)
        .add_startup_system(player_spawn)
        .add_startup_system(enemy_spawn)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_control)
                .with_system(player_bullet_spawn)
                .with_system(player_bullet_move)
        )
        .add_event::<CollisionEvent>()
        .insert_resource(BulletTimer(Timer::from_seconds(
            PLAYER_BULLET_COOLDOWN,
            true,
        )))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// Create windows related stuffs
fn access_window_system(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        window.set_title(String::from("Touhou Fangame"));
        window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
        window.set_resizable(false);
    }
}

fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn player_spawn(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOUR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(PLAYER_WIDTH, PLAYER_HEIGHT, 0.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(PlayerStatus {
            is_dead: false,
            is_shoot: false,
            is_focus: false,
        });
}

fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_state: Query<(&mut Transform, &mut PlayerStatus), With<Player>>,
) {
    for (mut player_transform, mut player_status) in player_state.iter_mut() {
        let mut direction_horizontal = 0.0;
        let mut direction_vertical = 0.0;

        player_status.is_focus = false;
        player_status.is_shoot = false;

        if keyboard_input.pressed(KeyCode::Left) {
            direction_horizontal -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_horizontal += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction_vertical -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_vertical += 1.0;
        }

        if keyboard_input.pressed(KeyCode::LShift) {
            player_status.is_focus = true;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            player_status.is_shoot = true;
        }

        if player_status.is_focus == true {
            direction_vertical = direction_vertical / FOCUS_SCALE;
            direction_horizontal = direction_horizontal / FOCUS_SCALE;
        }

        let new_player_x_position =
            player_transform.translation.x + direction_horizontal * PLAYER_SPEED * TIME_STEP;
        let new_player_y_position =
            player_transform.translation.y + direction_vertical * PLAYER_SPEED * TIME_STEP;

        player_transform.translation.x = new_player_x_position;
        player_transform.translation.y = new_player_y_position;
    }
}


fn player_bullet_move(mut bullet_state: Query<&mut Transform, With<PlayerBullet>>) {
    for mut bullet_position in bullet_state.iter_mut() {
        bullet_position.translation.y += BULLET_SPEED * TIME_STEP;
    }
}

fn enemy_bullet_spawn(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &EnemyStatus), With<Enemy>>,
) {
    // make the x and y axis of the bullet shoot in a cosine or sine wave pattern, no need for
    // circles
    for (enemy_transform, enemy_status) in enemy_query.iter() {
        let total_bullet_array: f32 = 1.;
        let bullet_per_array: f32 = 1.;
        let starting_angle: f32 = 1.;
        let spin_rate: f32 = 0.0;
        let spin_modifier: f32 = 0.0;
        let fire_rate: f32 = 0.0;
        let bullet_curve: f32 = f32::powf(1.0, 2.0);
        let x_offset = 0.0;
        let y_offset = 0.0;
        let degree = 60.0;

        let bullet_x = f32::cos(degree * enemy_transform.translation.x) - f32::sin(degree * enemy_transform.translation.y);
        let bullet_y = f32::sin(degree * enemy_transform.translation.x) + f32::sin(degree * enemy_transform.translation.y);

        //if enemy_status.is_shoot {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: ENEMY_COLOUR,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(bullet_x, bullet_y, 0.),
                        scale: Vec3::new(BULLET_WIDTH, BULLET_HEIGHT, 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(EnemyBullet);
        //}
    }
}
