mod model;
mod components {
    pub mod controllers;
    pub mod views;
}
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log::LogPlugin,
    prelude::*,
    window::{Cursor, PresentMode, WindowMode, WindowResolution},
};
use std::env;

fn main() {
    let mut path = env::current_exe()
        .unwrap_or_else(|e| panic!("Failed to get current executable path: {}", e));

    //remove executable name
    path.pop();
    if env::var("SEASON_SCHOLARS_DEV_ASSETS").is_ok() {
        path.pop();
        path.pop();
        path.pop();
    }

    println!("assets expected in {:?}", path);

    let mut debug_pointer_click: bool = false;
    let mut c = Config {
        base_path: path,
        students_init: 6,
        students_rows_nb: 3,
        long_action_s_min: 3.0,
        short_action_s_min: 1.0,
        long_action_s_max: 12.0,
        short_action_s_max: 3.0,
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
        volume: 1.0,
    };

    if env::var("SEASON_SCHOLARS_DEV").is_ok() {
        debug_pointer_click = true;
        c.debug_start_game_immediately = true;
    }
    let mut vsync: PresentMode = PresentMode::Fifo;
    if let Ok(env_vsync) = env::var("SEASON_SCHOLARS_DEV_VSYNC") {
        vsync = match env_vsync.as_str() {
            "AutoVsync" => PresentMode::AutoVsync,
            "AutoNoVsync" => PresentMode::AutoNoVsync,
            "Fifo" => PresentMode::Fifo,
            "FifoRelaxed" => PresentMode::FifoRelaxed,
            "Immediate" => PresentMode::Immediate,
            "Mailbox" => PresentMode::Mailbox,
            _ => {
                println!("Unknown PresentMode '{}', defaulting to Fifo", env_vsync);
                PresentMode::Fifo
            }
        };
    }

    let mut window_mode: WindowMode = WindowMode::Fullscreen;
    if let Ok(env_window_mode) = env::var("SEASON_SCHOLARS_DEV_WINDOW_MODE") {
        window_mode = match env_window_mode.as_str() {
            "Windowed" => WindowMode::Windowed,
            "Fullscreen" => WindowMode::Fullscreen,
            "BorderlessFullscreen" => WindowMode::BorderlessFullscreen,
            "SizedFullscreen" => WindowMode::SizedFullscreen,
            _ => {
                println!(
                    "Unknown WindowMode '{}', defaulting to Fullscreen",
                    env_window_mode
                );
                WindowMode::Fullscreen
            }
        };
    }
    read_env_var_to_i8(
        "SEASON_SCHOLARS_DEV_PORTAL_HEALTH",
        &mut c.portal_health_max,
    );
    read_env_var_to_f64(
        "SEASON_SCHOLARS_DEV_SHORT_ACTION_S_MIN",
        &mut c.short_action_s_min,
    );
    read_env_var_to_f64(
        "SEASON_SCHOLARS_DEV_SHORT_ACTION_S_MAX",
        &mut c.short_action_s_max,
    );
    read_env_var_to_f64(
        "SEASON_SCHOLARS_DEV_LONG_ACTION_S_MIN",
        &mut c.long_action_s_min,
    );
    read_env_var_to_f64(
        "SEASON_SCHOLARS_DEV_LONG_ACTION_S_MAX",
        &mut c.long_action_s_max,
    );
    read_env_var_to_f32("SEASON_SCHOLARS_DEV_VOLUME", &mut c.volume);

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
                        mode: window_mode,
                        present_mode: vsync,
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
        projection: OrthographicProjection {
            scale: 0.9,
            area: bevy::math::Rect {
                min: Vec2::new(-960.0, -540.0),
                max: Vec2::new(960.0, 540.0),
            },
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(1080.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(20., 50., 1000.),
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

fn read_env_var_to_f64(var_name: &str, target: &mut f64) {
    if let Ok(var_value) = env::var(var_name) {
        if !var_value.is_empty() {
            match var_value.parse::<f64>() {
                Ok(parsed_value) => *target = parsed_value,
                Err(e) => eprintln!("Failed to parse env var {}: {}", var_name, e),
            }
        }
    }
}

fn read_env_var_to_f32(var_name: &str, target: &mut f32) {
    if let Ok(var_value) = env::var(var_name) {
        if !var_value.is_empty() {
            match var_value.parse::<f32>() {
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
