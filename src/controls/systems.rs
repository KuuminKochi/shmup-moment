use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

use super::resources::KeyStateRes;

fn input_parser_system(
    mut inp_evr: EventReader<KeyboardInput>,
    mut states_res: ResMut<KeyStateRes>
) {
    for inp in inp_evr.iter() {
        match inp.key_code {
            Some(key) => {
                states_res.update(key, inp.state);
            },
            None => {}
        }
    }
}

