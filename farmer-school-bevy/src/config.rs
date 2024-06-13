use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Config {
    pub base_path: String,
    pub students_max: usize,
    pub long_action_s: f64,
    pub short_action_s: f64,
}
