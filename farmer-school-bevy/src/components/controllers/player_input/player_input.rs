use super::events::*;
use crate::{
    components::controllers::overlord::events::{
        DisplayScreenGameEvent, DisplayScreenGameOverRecapEvent, DisplayScreenMenuEvent,
    },
    model::definitions::*,
};
use bevy::prelude::*;

pub struct PlayerInputControllerPlugin;

impl Plugin for PlayerInputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>()
            .insert_resource(InputData {
                active_player_a: true,
                active_player_b: false,
            })
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
    data: Res<InputData>,
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

        if detect_input_changed(&keyboard_input, keys_to_check) {
            let out = input_player_a(&keyboard_input);
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

        if detect_input_changed(&keyboard_input, keys_to_check) {
            let out = input_player_b(&keyboard_input);
            trace!("player B {:?}", out);
            input_events.send(out);
        }
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
        ..default()
    };

    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        out.short_action = true;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight)
    {
        out.long_action = true;
    }
    if keyboard_input.pressed(KeyCode::AltLeft)
        || keyboard_input.pressed(KeyCode::AltRight)
        || keyboard_input.pressed(KeyCode::Enter)
    {
        out.confirm_move = true;
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
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::B),
        ..default()
    };

    if keyboard_input.pressed(KeyCode::KeyQ)
        || keyboard_input.pressed(KeyCode::Numpad0)
        || keyboard_input.pressed(KeyCode::NumpadSubtract)
        || keyboard_input.pressed(KeyCode::PageUp)
    {
        out.short_action = true;
    }
    if keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::NumpadAdd)
        || keyboard_input.pressed(KeyCode::PageDown)
    {
        out.long_action = true;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::NumpadEnter) {
        out.confirm_move = true;
    }

    if keyboard_input.pressed(KeyCode::KeyF) || keyboard_input.pressed(KeyCode::Numpad2) {
        out.direction += Vec2::new(0.0, -1.0)
    }
    if keyboard_input.pressed(KeyCode::KeyR) || keyboard_input.pressed(KeyCode::Numpad8) {
        out.direction += Vec2::new(0.0, 1.0)
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::Numpad4) {
        out.direction += Vec2::new(-1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::KeyG) || keyboard_input.pressed(KeyCode::Numpad6) {
        out.direction += Vec2::new(1.0, 0.0)
    }

    return out;
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
            assert_eq!(event.short_action, true);
            assert_eq!(event.direction, Vec2::new(0.0, 1.0));
        }
    }
}

#[derive(Resource)]
pub struct InputData {
    active_player_a: bool,
    active_player_b: bool,
}