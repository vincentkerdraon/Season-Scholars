mod model;
mod components {
    pub mod player_input;
    pub mod season;
    pub mod teacher;
    pub mod welcome;
}

use bevy::prelude::*;
use model::events::*;

fn main() {
    App::new()
        .add_event::<GraduateEvent>()
        .add_event::<GraduatedEvent>()
        .add_event::<TeachEvent>()
        .add_event::<TaughtEvent>()
        .add_event::<ObservePortal>()
        .add_event::<PortalObserved>()
        .add_event::<PortalAttackedEvent>()
        .add_event::<MonsterFedEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<ResetGameEvent>()
        .add_event::<InvalidActionStation>()
        .add_plugins(DefaultPlugins)
        .add_plugins(components::season::season::SeasonPlugin)
        .add_plugins(components::welcome::welcome::WelcomePlugin)
        .add_plugins(components::teacher::teacher::TeacherPlugin)
        .add_plugins(components::player_input::player_input::PlayerInputPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Hello",
            TextStyle {
                font_size: 60.0,
                ..default()
            },
        ),
        ..default()
    });
}
