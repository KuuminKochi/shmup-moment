//use crate::{game_flow::GameState, PIXEL2UNIT_MULTIPLIER};
use bevy::{prelude::*, sprite::collide_aabb::collide};

use self::systems::player_bullet_enemy::*;
//use iyes_loopless::prelude::*;

pub mod colliders;
pub mod components;
pub mod systems;
pub mod traits;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(player_bullet_enemy)
        .add_system(enemy_player_bullet);
        // this is just example with iyes' crates. i forgor their names
        // app.add_system_set(
        //     ConditionSet::new()
        //         .run_in_state(GameState::InGame)
        //         .with_system(test_collision_system::<BoxCollider, CircleCollider>)
        //         .into(),
        // );
    }
}
