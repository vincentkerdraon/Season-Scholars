mod config;
mod model;
mod components {
    pub mod controllers {
        pub mod overlord;
        pub mod player_input;
        pub mod portal;
        pub mod season;
        pub mod teacher;
        pub mod welcome;
    }
    pub mod views {
        pub mod menu;
        pub mod recap;
        pub mod room;
        pub mod welcome;
    }
}

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log::LogPlugin,
    prelude::*,
    window::{Cursor, WindowResolution},
};
use config::Config;
use model::events::*;

fn main() {
    App::new()
        .insert_resource(Config {
            base_path: "/home/korrident/Documents/farmer-school/images/ready/".to_string(),
            students_max: 9,
            long_action_s: 2.0,  //FIXME
            short_action_s: 1.0, //FIXME
            portal_health_max: 10,
            seasons_duration_s: 10.,
        })
        .add_event::<GraduateEvent>()
        .add_event::<GraduatedEvent>()
        .add_event::<TeachEvent>()
        .add_event::<TaughtEvent>()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin { ..default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Season Scholars".to_string(),
                        resolution: WindowResolution::new(1920., 1080.),
                        resizable: true, //FIXME
                        cursor: Cursor {
                            // visible: false, //FIXME
                            ..default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(components::controllers::overlord::overlord::OverlordPlugin)
        .add_plugins(components::controllers::season::season::SeasonPlugin)
        .add_plugins(components::controllers::welcome::welcome::WelcomePlugin)
        .add_plugins(components::controllers::teacher::teacher::TeacherPlugin)
        .add_plugins(components::controllers::player_input::player_input::PlayerInputPlugin)
        .add_plugins(components::controllers::portal::portal::PortalPlugin)
        .add_plugins(components::views::room::room::RoomPlugin)
        .add_plugins(components::views::welcome::welcome::WelcomePlugin)
        .add_plugins(components::views::menu::menu::MenuPlugin)
        .add_plugins(components::views::recap::recap::RecapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, print_mouse_click_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            ..default()
        },
        ..default()
    });
}

fn print_mouse_click_events(
    windows: Query<&Window>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(clicked) = mouse_button_input_events.read().last() {
        if clicked.button == MouseButton::Left && clicked.state == ButtonState::Pressed {
            if let Some(moved) = cursor_moved_events.read().last() {
                let window = windows.single();

                for (camera, camera_transform) in query.iter() {
                    if let Some(world_position) =
                        screen_to_world(window, camera, camera_transform, moved.position)
                    {
                        let clicked_fixed = Vec2 {
                            x: world_position.x,
                            y: -world_position.y,
                        };
                        println!("World position {:?}", clicked_fixed);
                    }
                }
            }
        }
    }
}

fn screen_to_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    screen_position: Vec2,
) -> Option<Vec2> {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE; // normalize coordinates to range [-1, 1]
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
    Some(world_position.truncate())
}
