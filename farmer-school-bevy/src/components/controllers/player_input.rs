use crate::model::overlord::*;
use crate::model::player_input::*;
use bevy::prelude::*;

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
        if let Some(out) = input_player_a(&keyboard_input) {
            trace!("player A {:?}", out);
            input_events.send(out);
        }
    }

    if data.active_player_b {
        if let Some(out) = input_player_b(&keyboard_input) {
            trace!("player B {:?}", out);
            input_events.send(out);
        }
    }
}

fn input_player_a(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Option<PlayerInputEvent> {
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::A),
        ..default()
    };

    let mut found = false;

    if keyboard_input.just_pressed(KeyCode::ShiftLeft)
        || keyboard_input.just_pressed(KeyCode::ShiftRight)
    {
        out.short_action = true;
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::ControlLeft)
        || keyboard_input.just_pressed(KeyCode::ControlRight)
    {
        out.long_action = true;
        found = true;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        out.direction += Vec2::new(0.0, -1.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        out.direction += Vec2::new(0.0, 1.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        out.direction += Vec2::new(-1.0, 0.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        out.direction += Vec2::new(1.0, 0.0);
        found = true;
    }
    if found {
        return Some(out);
    }
    None
}

fn input_player_b(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Option<PlayerInputEvent> {
    let mut out: PlayerInputEvent = PlayerInputEvent {
        teacher: (Teacher::B),
        ..default()
    };

    let mut found = false;

    if keyboard_input.just_pressed(KeyCode::KeyQ)
        || keyboard_input.just_pressed(KeyCode::Numpad0)
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
        || keyboard_input.just_pressed(KeyCode::PageUp)
    {
        out.short_action = true;
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::KeyW)
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
        || keyboard_input.just_pressed(KeyCode::PageDown)
    {
        out.long_action = true;
        found = true;
    }

    if keyboard_input.just_pressed(KeyCode::KeyF) || keyboard_input.just_pressed(KeyCode::Numpad2) {
        out.direction += Vec2::new(0.0, -1.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) || keyboard_input.just_pressed(KeyCode::Numpad8) {
        out.direction += Vec2::new(0.0, 1.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::Numpad4) {
        out.direction += Vec2::new(-1.0, 0.0);
        found = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyG) || keyboard_input.just_pressed(KeyCode::Numpad6) {
        out.direction += Vec2::new(1.0, 0.0);
        found = true;
    }
    if found {
        return Some(out);
    }
    None
}

pub struct PlayerInputControllerPlugin;

impl Plugin for PlayerInputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>()
            .insert_resource(InputData { ..default() })
            .add_systems(PreUpdate, listen_events)
            .add_systems(PreUpdate, input_system);
    }
}

#[derive(Resource, Default)]
struct InputData {
    active_player_a: bool,
    active_player_b: bool,
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
