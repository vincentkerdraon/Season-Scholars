use crate::model::definitions::*;
use bevy::prelude::*;

///Start from zero
#[derive(Event, Debug)]
pub struct ResetGameEvent {
    pub teachers: Vec<Teacher>,
}

#[derive(Event, Debug)]
pub struct GameOverEvent {
    pub reason: String,
}

#[derive(Event, Debug, Default, Clone)]
pub struct DisplayScreenGameOverRecapEvent {
    pub teachers: Vec<Teacher>,
    pub score: i32,
    pub reason: String,
    pub time_since_start_s: f64,
    pub seasons_elapsed: i64,
}

#[derive(Event, Debug)]
pub struct DisplayScreenGameEvent {
    pub teachers: Vec<Teacher>,
}

#[derive(Event, Debug)]
pub struct DisplayScreenMenuEvent {
    pub teachers: Vec<Teacher>,
}

/// Functional error because the requested action is not possible
#[derive(Event, Debug)]
pub struct InvalidActionStationEvent {
    pub station: Station,
    pub teacher: Teacher,
}

/// Functional error because the requested move is not possible
#[derive(Event, Debug)]
pub struct InvalidMoveEvent {
    pub station: Station,
    pub teacher: Teacher,
}