mod model;
mod components {
    pub mod teacher;
    pub mod welcome;
}

use bevy::prelude::*;
use model::events::*;

fn main() {
    App::new()
        //   .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        //         // Uncomment this to override the default log settings:
        //         // level: bevy::log::Level::TRACE,
        //         // filter: "wgpu=warn,bevy_ecs=info".to_string(),
        //         ..default()
        //     }))
        // .add_systems(Update, log_system)
        .add_event::<GraduateEvent>()
        .add_event::<GraduatedEvent>()
        .add_event::<TeachEvent>()
        .add_event::<TaughtEvent>()
        .add_event::<ObservePortal>()
        .add_event::<PortalObserved>()
        .add_event::<PortalAttackedEvent>()
        .add_event::<MonsterFedEvent>()
        .add_event::<SeasonChangedEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<InvalidActionStation>()
        .add_event::<PlayerInput>()
        .add_plugins(DefaultPlugins)
        .add_plugins(components::welcome::welcome::WelcomePlugin)
        .add_plugins(components::teacher::teacher::TeacherPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, debug_event_system)
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

// fn log_system() {
//     // here is how you write new logs at each "log level" (in "least important" to "most important"
//     // order)
//     trace!("very noisy");
//     debug!("helpful for debugging");
//     info!("helpful information that is worth printing by default");
//     warn!("something bad happened that isn't a failure, but thats worth calling out");
//     error!("something failed");

//     // by default, trace and debug logs are ignored because they are "noisy"
//     // you can control what level is logged by setting up the LogPlugin
//     // alternatively you can set the log level via the RUST_LOG=LEVEL environment variable
//     // ex: RUST_LOG=trace, RUST_LOG=info,bevy_ecs=warn
//     // the format used here is super flexible. check out this documentation for more info:
//     // https://docs.rs/tracing-subscriber/*/tracing_subscriber/filter/struct.EnvFilter.html
// }

pub fn debug_event_system(mut player_input_events: EventWriter<PlayerInput>) {
    info!("debug_event_system starting");
    player_input_events.send(PlayerInput {
        teacher: model::definitions::Teacher::A,
        direction: model::definitions::Direction::Down,
        long_action: true,
        short_action: false,
    });
}
