use bevy::{prelude::*, time::FixedTimestep};

// GAME CONSTANT
const TIME_STEP: f32 = 1.0 / 60.0;

// PLAYER
const PLAYER_COLOUR: Color = Color::rgb(255., 23., 23.);
const PLAYER_WIDTH: f32 = 25.;
const PLAYER_HEIGHT: f32 = 25.;
const PLAYER_SPEED: f32 = 500.;

// PLAYER CONTROL
const FOCUS_SCALE: f32 = 2.;

// BULLET
const BULLET_COLOUR: Color = Color::rgb(0.0, 0.0, 255.);
const BULLET_WIDTH: f32 = 2.;
const BULLET_HEIGHT: f32 = 2.;
const BULLET_SPEED: f32 = 600.;

// COMPONENTS
#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerStatus {
    is_dead: bool,
    is_shoot: bool,
    is_focus: bool,
}

#[derive(Component)]
struct Bullet;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(access_window_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_control)
                .with_system(bullet_spawn)
                .with_system(bullet_move),
        )
        .add_startup_system(player_spawn)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// Create windows related stuffs
fn access_window_system(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        window.set_title(String::from("Touhou Fangame"));
        window.set_resolution(620., 480.);
        window.set_resizable(false);
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

fn bullet_spawn(
    mut commands: Commands,
    player_state: Query<&PlayerStatus, With<Player>>
    ) {
    let player_status = player_state.single();

    if player_status.is_shoot == true {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: BULLET_COLOUR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(BULLET_WIDTH, BULLET_HEIGHT, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Bullet);
    }
}

fn bullet_move(
    mut bullet_state: Query<&mut Transform, With<Bullet>>,
    ) {
    let mut bullet_position = bullet_state.single_mut();
        bullet_position.translation.y += BULLET_SPEED;
}
