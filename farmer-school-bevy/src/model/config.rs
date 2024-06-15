use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Config {
    pub base_path: String,
    pub students_max: usize,
    pub students_init: usize,
    pub long_action_s: f64,
    pub short_action_s: f64,
    pub portal_health_max: i8,
    pub seasons_duration_s: f64,
    pub portal_opened_nb: i8,
    pub portal_closed_nb: i8,
}
