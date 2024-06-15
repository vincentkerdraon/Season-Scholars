mod model;
mod components {
    pub mod moves;
    pub mod teacher_busy;
    pub mod controllers {
        pub mod overlord;
        pub mod player_input;
        pub mod portal;
        pub mod season;
        pub mod students;
        pub mod teacher;
        pub mod welcome;
    }
    pub mod views {
        pub mod menu;
        pub mod portal;
        pub mod recap;
        pub mod room;
        pub mod student;
        pub mod teacher;
        pub mod welcome;
    }
}

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log::LogPlugin,
    prelude::*,
    window::{Cursor, WindowResolution},
};

fn main() {
    let mut app: App = App::new();
    app.insert_resource(Config {
        base_path: "/home/korrident/Documents/farmer-school/images/ready/".to_string(),
        students_max: 9,
        students_init: 6,
        students_center_nb: 5,
        students_side_nb: 6,
        long_action_s: 5.0,  //FIXME
        short_action_s: 2.0, //FIXME
        portal_health_max: 10,
        seasons_duration_s: 10.,
        portal_opened_nb: 5,
        portal_closed_nb: 10,
    })
    .add_plugins(
        DefaultPlugins
            .set(LogPlugin { ..default() })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Season Scholars".to_string(),
                    resolution: WindowResolution::new(1920., 1080.),
                    resizable: false,
                    cursor: Cursor {
                        // visible: false, //FIXME
                        ..default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
    )
    .add_plugins(components::controllers::overlord::overlord::OverlordControllerPlugin)
    .add_plugins(components::controllers::season::season::SeasonControllerPlugin)
    .add_plugins(components::controllers::welcome::welcome::WelcomeControllerPlugin)
    .add_plugins(components::controllers::teacher::teacher::TeacherControllerPlugin)
    .add_plugins(components::controllers::player_input::player_input::PlayerInputControllerPlugin)
    .add_plugins(components::controllers::portal::portal::PortalControllerPlugin)
    .add_plugins(components::controllers::students::students::StudentsControllerPlugin)
    .add_plugins(components::views::room::room::RoomViewPlugin)
    .add_plugins(components::views::welcome::welcome::WelcomeViewPlugin)
    .add_plugins(components::views::teacher::teacher::TeacherViewPlugin)
    .add_plugins(components::views::menu::menu::MenuViewPlugin)
    .add_plugins(components::views::recap::recap::RecapViewPlugin)
    .add_plugins(components::views::portal::portal::PortalViewPlugin)
    .add_plugins(components::views::student::student::StudentViewPlugin)
    .add_systems(Startup, setup);

    #[cfg(debug_assertions)]
    {
        app.add_systems(Update, _log_mouse_click);
        // app.add_systems(Update, log_fps);

        // use bevy::diagnostic::LogDiagnosticsPlugin;
        // app.add_plugins(LogDiagnosticsPlugin {
        //     debug: true,
        //     ..default()
        // });
    }

    app.run();
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

fn _log_mouse_click(
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

use model::config::Config;
use rand::Rng;

fn _log_fps(time: Res<Time>) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..=1000) == 0 {
        let d = time.delta_seconds();
        debug!("FPS={}", (1.0 / d).round());
    }
}
