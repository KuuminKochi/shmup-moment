// use external library
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

// mods
mod collision;
mod controls;

// COMPONENTS

#[derive(Component)]
enum Spellblocks {
    Circle,
}

impl Spellblocks {
    fn circle(
        commands: Commands,
        enemy_query: Query<&Transform, With<Enemy>>,
        ){
        
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerStatus {
    is_dead: bool,
    is_shoot: bool,
    is_focus: bool,
}

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct BulletTimer(Timer);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyStatus {
    is_dead: bool,
    is_shoot: bool,
}

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;


// GAME CONSTANT
const SCREEN_WIDTH: f32 = 640.;
const SCREEN_HEIGHT: f32 = 480.;
const TIME_STEP: f32 = 1.0 / 60.0;

// PLAYER
const PLAYER_COLOUR: Color = Color::rgb(255., 23., 23.);
const PLAYER_WIDTH: f32 = 25.;
const PLAYER_HEIGHT: f32 = 25.;
const PLAYER_SPEED: f32 = 500.;
const FOCUS_SCALE: f32 = 2.;

// BULLET
const BULLET_COLOUR: Color = Color::rgb(0.0, 0.0, 255.);
const PLAYER_BULLET_COOLDOWN: f32 = 0.3;
const BULLET_WIDTH: f32 = 20.;
const BULLET_HEIGHT: f32 = 20.;
const BULLET_SPEED: f32 = 600.;

// ENEMY
const ENEMY_COLOUR: Color = Color::rgb(0.0, 255., 255.);
const ENEMY_WIDTH: f32 = 2.;
const ENEMY_HEIGHT: f32 = 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(access_window_system)
        .add_startup_system(player_spawn)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_control)
                .with_system(player_bullet_spawn)
                .with_system(player_bullet_move)
                .with_system(enemy_spawn)
                .with_system(check_for_collisions)
                .with_system(enemy_bullet_spawn)
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
        .insert(Collider)
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

fn player_bullet_spawn(
    mut commands: Commands,
    player_state: Query<(&Transform, &PlayerStatus), With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<BulletTimer>,
) {
    for (player_position, player_status) in player_state.iter() {
        if player_status.is_shoot == true && timer.0.tick(time.delta()).just_finished() {
            commands .spawn() .insert_bundle(SpriteBundle { sprite: Sprite {
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
                .insert(PlayerBullet)
                .insert(Collider);

            timer.0.reset();
        }
    }
}

fn player_bullet_move(mut bullet_state: Query<&mut Transform, With<PlayerBullet>>) {
    for mut bullet_position in bullet_state.iter_mut() {
        bullet_position.translation.y += BULLET_SPEED * TIME_STEP;
    }
}

fn enemy_spawn(mut commands: Commands) {
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
        .insert(Collider)
        .insert(EnemyStatus {
            is_dead: false,
            is_shoot: false,
        });
}

fn check_for_collisions(
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, player_bullet_transform) in player_bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            
            let collision = collide(
                player_bullet_transform.translation,
                player_bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => {}
                    Collision::Right => {}
                    Collision::Top => {}
                    Collision::Bottom => {}
                    Collision::Inside => { 
                        commands.entity(enemy_entity).despawn();
                        commands.entity(bullet_entity).despawn();
                    }
                };
            }
        }
    }
}

fn enemy_bullet_spawn (
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    ) {

    }




















