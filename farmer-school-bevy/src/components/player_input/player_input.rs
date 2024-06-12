use super::events::*;
use crate::model::definitions::*;
use bevy::prelude::*;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>()
            .add_systems(Update, input_system);
    }
}

fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut input_events: EventWriter<PlayerInputEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape)
        || mouse_button_input.just_pressed(MouseButton::Middle)
    {
        warn!("//FIXME display menu");
        return;
    }

    let keys_to_check = vec![
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ShiftRight,
        KeyCode::ShiftRight,
    ];

    if detect_input_changed(&keyboard_input, keys_to_check) {
        let out = input_player_a(&keyboard_input);
        warn!("detect_input_changed A {:?}", out);
        input_events.send(out);
    }

    let keys_to_check = vec![
        KeyCode::Numpad8,
        KeyCode::Numpad4,
        KeyCode::Numpad2,
        KeyCode::Numpad6,
        KeyCode::Numpad0,
        KeyCode::NumpadSubtract,
        KeyCode::NumpadAdd,
        KeyCode::KeyR,
        KeyCode::KeyD,
        KeyCode::KeyF,
        KeyCode::KeyG,
        KeyCode::KeyQ,
        KeyCode::KeyA,
        KeyCode::KeyW,
        KeyCode::PageUp,
        KeyCode::PageDown,
    ];

    if detect_input_changed(&keyboard_input, keys_to_check) {
        let out = input_player_b(&keyboard_input);
        input_events.send(out);
    }
}

fn detect_input_changed(keyboard_input: &Res<ButtonInput<KeyCode>>, keys: Vec<KeyCode>) -> bool {
    for key in keys.iter() {
        if keyboard_input.just_pressed(*key) || keyboard_input.just_released(*key) {
            return true;
        }
    }
    return false;
}

fn input_player_a(keyboard_input: &Res<ButtonInput<KeyCode>>) -> PlayerInputEvent {
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::A),
        direction: Vec2::ZERO,
        short_action: false,
        long_action: false,
    };

    if keyboard_input.pressed(KeyCode::ShiftRight) {
        out.short_action = true;
    }
    if keyboard_input.pressed(KeyCode::ControlRight) {
        out.long_action = true;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        out.direction += Vec2::new(0.0, -1.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        out.direction += Vec2::new(0.0, 1.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        out.direction += Vec2::new(-1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        out.direction += Vec2::new(1.0, 0.0)
    }

    return out;
}

fn input_player_b(keyboard_input: &Res<ButtonInput<KeyCode>>) -> PlayerInputEvent {
    //FIXME
    return PlayerInputEvent {
        teacher: (Teacher::B),
        direction: Vec2::ZERO,
        short_action: false,
        long_action: false,
    };
}
