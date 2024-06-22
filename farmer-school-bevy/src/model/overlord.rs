use super::teacher::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ResetGameStep1Event {
    pub teachers: Vec<(Teacher, Station, ActionShortDuration, ActionLongDuration)>,
}
#[derive(Event, Debug)]
pub struct ResetGameStep2Event {
    pub teachers: Vec<(Teacher, Station, ActionShortDuration, ActionLongDuration)>,
}
#[derive(Event, Debug)]
pub struct ResetGameStep3Event {}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    Menu = 1,
    Game,
    GameOverRecap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Station {
    #[default]
    None,
    StudentLeft,
    StudentCenter,
    StudentRight,
    Welcome,
    Portal,
    Kitchen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Teacher {
    #[default]
    A = 1,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reaction {
    Long = 1,
    Short,
    Fail,
}

/// Functional error because the requested move is not possible
#[derive(Event, Debug, Default)]
pub struct ComponentReady {
    pub listen_data_events: bool,
    pub listen_player_input: bool,
}
