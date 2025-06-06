use std::path::PathBuf;

use bevy::prelude::*;

pub const VERSION: &str = "v1.6";
pub const REPO: &str = "https://gitlab.com/eclypsaine/farmer-school";

#[derive(Debug, Clone, Resource)]
pub struct Config {
    pub base_path: PathBuf,
    ///starting number of students
    pub students_init: usize,
    pub students_rows_nb: i8,
    pub long_action_s_min: f64,
    pub short_action_s_min: f64,
    pub long_action_s_max: f64,
    pub short_action_s_max: f64,
    ///how often the teachers get more tired
    pub actions_increase_delta: f64,
    ///increase of teacher tiredness
    pub actions_increase_increment: f64,
    pub seasons_duration_s: f64,
    pub portal_health_max: i8,
    pub portal_windows_nb: i8,
    pub portal_windows_seasons_nb: i8,
    pub food_max: i8,
    pub draw_frame_modulo: i8,
    ///silence between audio tracks
    pub track_break_s: f64,
    pub debug_start_game_immediately: bool,
    pub debug_disable_student_eating: bool,
    pub debug_disable_season_monster: bool,
    pub volume: f32,
}
