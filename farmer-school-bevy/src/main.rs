mod model;
mod components {
    pub mod controllers;
    pub mod views;
}
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log::LogPlugin,
    prelude::*,
    window::{Cursor, WindowResolution},
};
use std::env;

fn main() {
    let mut path = env::current_exe()
        .unwrap_or_else(|e| panic!("Failed to get current executable path: {}", e));

    if env::var("SEASON_SCHOLARS_DEV_ASSETS").is_ok() {
        path.pop();
        path.pop();
        path.pop();
        path.pop();
    }

    let mut debug_pointer_click: bool = false;
    let mut c = Config {
        base_path: path,
        students_init: 6,
        students_rows_nb: 3,
        long_action_s_min: 6.0,
        short_action_s_min: 2.0,
        long_action_s_max: 12.0,
        short_action_s_max: 6.0,
        actions_increase_delta: 10.,
        actions_increase_increment: 0.5,
        seasons_duration_s: 12.,
        portal_health_max: 4,
        portal_windows_nb: 4,
        portal_windows_seasons_nb: 3,
        food_max: 5,
        draw_frame_modulo: 20,
        track_break_s: 5.0,
        debug_start_game_immediately: false,
        debug_disable_student_eating: false,
        debug_disable_season_monster: false,
    };

    if env::var("SEASON_SCHOLARS_DEV").is_ok() {
        debug_pointer_click = true;
        c.debug_start_game_immediately = true;
    }
    read_env_var_to_i8(
        "SEASON_SCHOLARS_DEV_PORTAL_HEALTH",
        &mut c.portal_health_max,
    );
    if env::var("SEASON_SCHOLARS_DEV_STUDENT_NOT_EATING").is_ok() {
        c.debug_disable_student_eating = true;
    }
    if env::var("SEASON_SCHOLARS_DEV_NO_SEASON_MONSTER").is_ok() {
        c.debug_disable_season_monster = true;
    }

    let mut app: App = App::new();
    app.insert_resource(c)
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin { ..default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Season Scholars".to_string(),
                        resolution: WindowResolution::new(1920., 1080.),
                        resizable: true,
                        cursor: Cursor {
                            visible: debug_pointer_click,
                            ..default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(components::controllers::overlord::OverlordControllerPlugin)
        .add_plugins(components::controllers::season::SeasonControllerPlugin)
        .add_plugins(components::controllers::welcome::WelcomeControllerPlugin)
        .add_plugins(components::controllers::teacher::TeacherControllerPlugin)
        .add_plugins(components::controllers::portal::PortalControllerPlugin)
        .add_plugins(components::controllers::students::StudentsControllerPlugin)
        .add_plugins(components::controllers::kitchen::KitchenControllerPlugin)
        .add_plugins(components::controllers::player_input::PlayerInputControllerPlugin)
        .add_plugins(components::views::room::RoomViewPlugin)
        .add_plugins(components::views::welcome::WelcomeViewPlugin)
        .add_plugins(components::views::teacher::TeacherViewPlugin)
        .add_plugins(components::views::menu::MenuViewPlugin)
        .add_plugins(components::views::recap::RecapViewPlugin)
        .add_plugins(components::views::portal::PortalViewPlugin)
        .add_plugins(components::views::student::StudentViewPlugin)
        .add_plugins(components::views::kitchen::KitchenViewPlugin)
        .add_plugins(components::views::sound::SoundViewPlugin)
        .add_systems(Startup, setup);

    if debug_pointer_click {
        app.add_systems(Update, _log_mouse_click);
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

fn read_env_var_to_i8(var_name: &str, target: &mut i8) {
    if let Ok(var_value) = env::var(var_name) {
        if !var_value.is_empty() {
            match var_value.parse::<i8>() {
                Ok(parsed_value) => *target = parsed_value,
                Err(e) => eprintln!("Failed to parse env var {}: {}", var_name, e),
            }
        }
    }
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
    let window_size = Vec2::new(window.width(), window.height());
    let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;
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
