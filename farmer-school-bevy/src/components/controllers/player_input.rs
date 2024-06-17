use crate::model::overlord::*;
use crate::model::player_input::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub struct PlayerInputControllerPlugin;

impl Plugin for PlayerInputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>()
            .insert_resource(InputData { ..default() })
            .add_systems(PreUpdate, listen_events)
            .add_systems(PreUpdate, input_system);
    }
}

fn listen_events(
    mut data: ResMut<InputData>,
    mut display_screen_game_events: EventReader<DisplayScreenGameEvent>,
    mut display_screen_menu_events: EventReader<DisplayScreenMenuEvent>,
) {
    if let Some(e) = display_screen_game_events.read().last() {
        data.active_player_a = e.teachers.contains(&Teacher::A);
        data.active_player_b = e.teachers.contains(&Teacher::B);
    }
    if display_screen_menu_events.read().last().is_some() {
        data.active_player_a = true;
        data.active_player_b = true;
    }
}

fn input_system(
    mut data: ResMut<InputData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut input_events: EventWriter<PlayerInputEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape)
        || mouse_button_input.just_pressed(MouseButton::Middle)
    {
        let emit: PlayerInputEvent = PlayerInputEvent {
            reset: true,
            ..default()
        };
        trace!("{:?}", emit);
        input_events.send(emit);
        return;
    }

    if data.active_player_a {
        let keys_to_check = vec![
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ShiftLeft,
            KeyCode::ControlLeft,
            KeyCode::AltLeft,
            KeyCode::ShiftRight,
            KeyCode::ControlRight,
            KeyCode::AltRight,
            KeyCode::Enter,
        ];

        if detect_input_changed(&mut data.keys_pressed, &keyboard_input, keys_to_check) {
            let out = input_player_a(&data.keys_pressed);
            trace!("player A {:?}", out);
            input_events.send(out);
        }
    }

    if data.active_player_b {
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
            KeyCode::KeyS,
            KeyCode::NumpadEnter,
            //FIXME move_confirm in menu
        ];

        if detect_input_changed(&mut data.keys_pressed, &keyboard_input, keys_to_check) {
            let out = input_player_b(&data.keys_pressed);
            trace!("player B {:?}", out);
            input_events.send(out);
        }
    }
}

fn detect_input_changed(
    keys_pressed: &mut HashSet<KeyCode>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    keys: Vec<KeyCode>,
) -> bool {
    let mut updated = false;
    for key in keys.iter() {
        if keyboard_input.just_pressed(*key) {
            keys_pressed.insert(*key);
            updated = true;
        }

        if keyboard_input.just_released(*key) {
            keys_pressed.remove(key);
            updated = true;
        }
    }
    updated
}

fn input_player_a(keys_pressed: &HashSet<KeyCode>) -> PlayerInputEvent {
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::A),
        ..default()
    };

    if keys_pressed.get(&KeyCode::ShiftLeft).is_some()
        || keys_pressed.get(&KeyCode::ShiftRight).is_some()
    {
        out.short_action = true;
    }
    if keys_pressed.get(&KeyCode::ControlLeft).is_some()
        || keys_pressed.get(&KeyCode::ControlRight).is_some()
    {
        out.long_action = true;
    }
    if keys_pressed.get(&KeyCode::AltLeft).is_some()
        || keys_pressed.get(&KeyCode::AltRight).is_some()
        || keys_pressed.get(&KeyCode::Enter).is_some()
    {
        out.confirm_move = true;
    }
    if keys_pressed.get(&KeyCode::ArrowDown).is_some() {
        out.direction += Vec2::new(0.0, -1.0)
    }
    if keys_pressed.get(&KeyCode::ArrowUp).is_some() {
        out.direction += Vec2::new(0.0, 1.0)
    }
    if keys_pressed.get(&KeyCode::ArrowLeft).is_some() {
        out.direction += Vec2::new(-1.0, 0.0)
    }
    if keys_pressed.get(&KeyCode::ArrowRight).is_some() {
        out.direction += Vec2::new(1.0, 0.0)
    }

    out
}

fn input_player_b(keys_pressed: &HashSet<KeyCode>) -> PlayerInputEvent {
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::B),
        ..default()
    };

    if keys_pressed.get(&KeyCode::KeyQ).is_some()
        || keys_pressed.get(&KeyCode::Numpad0).is_some()
        || keys_pressed.get(&KeyCode::NumpadSubtract).is_some()
        || keys_pressed.get(&KeyCode::PageUp).is_some()
    {
        out.short_action = true;
    }
    if keys_pressed.get(&KeyCode::KeyA).is_some()
        || keys_pressed.get(&KeyCode::KeyW).is_some()
        || keys_pressed.get(&KeyCode::NumpadAdd).is_some()
        || keys_pressed.get(&KeyCode::PageDown).is_some()
    {
        out.long_action = true;
    }

    if keys_pressed.get(&KeyCode::KeyS).is_some()
        || keys_pressed.get(&KeyCode::NumpadEnter).is_some()
    {
        out.confirm_move = true;
    }

    if keys_pressed.get(&KeyCode::KeyF).is_some() || keys_pressed.get(&KeyCode::Numpad2).is_some() {
        out.direction += Vec2::new(0.0, -1.0)
    }
    if keys_pressed.get(&KeyCode::KeyR).is_some() || keys_pressed.get(&KeyCode::Numpad8).is_some() {
        out.direction += Vec2::new(0.0, 1.0)
    }
    if keys_pressed.get(&KeyCode::KeyD).is_some() || keys_pressed.get(&KeyCode::Numpad4).is_some() {
        out.direction += Vec2::new(-1.0, 0.0)
    }
    if keys_pressed.get(&KeyCode::KeyG).is_some() || keys_pressed.get(&KeyCode::Numpad6).is_some() {
        out.direction += Vec2::new(1.0, 0.0)
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::event::Events;

    #[test]
    fn test_input_player_a() {
        let mut app = App::new();
        app.add_plugins(PlayerInputControllerPlugin);

        app.world.insert_resource(ButtonInput::<KeyCode>::default());
        app.world
            .insert_resource(ButtonInput::<MouseButton>::default());

        let mut keyboard_input = app
            .world
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .unwrap();
        keyboard_input.press(KeyCode::ShiftLeft);
        keyboard_input.press(KeyCode::ArrowUp);

        app.update();

        let input_events = app
            .world
            .get_resource_mut::<Events<PlayerInputEvent>>()
            .unwrap();
        let mut event_reader = input_events.get_reader();

        for event in event_reader.read(&input_events) {
            assert_eq!(event.teacher, Teacher::A);
            assert!(event.short_action);
            assert_eq!(event.direction, Vec2::new(0.0, 1.0));
        }
    }
}

#[derive(Resource, Default)]
struct InputData {
    active_player_a: bool,
    active_player_b: bool,
    keys_pressed: HashSet<KeyCode>,
}
