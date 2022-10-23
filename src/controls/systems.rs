use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

fn input_event_reader_system(
    mut inp_evr: EventReader<KeyboardInput>,
    
) {
    for inp in inp_evr.iter() {
        match inp.key_code {
            Some(key) => {
                input_parser(key, inp.state);
            },
            None => {}
        }
    }
}

fn input_parser(key: KeyCode, state: ButtonState) {
    match key {
        KeyCode::Left => {
            
        },
        KeyCode::Right => {

        },
        KeyCode::Up => {

        },
        KeyCode::Down => {

        },
        _ => { /* do nothing */ }
    }
}

