use bevy::prelude::*;

use super::overlord::{Station, Teacher};

#[derive(Event, Debug)]
pub struct MoveTeacherEvent {
    pub station_from: Station,
    pub station_to: Station,
    pub teacher: Teacher,
}

#[derive(Event, Debug)]
pub struct TeacherMovedEvent {
    pub station_from: Station,
    pub station_to: Station,
    pub teacher: Teacher,
}

//TeacherSpeed is action duration in seconds
pub type ActionLongDuration = f64;
pub type ActionShortDuration = f64;

#[derive(Event, Debug)]
pub struct TeacherTiredEvent {
    pub short_action: ActionShortDuration,
    pub long_action: ActionLongDuration,
    pub teacher: Teacher,
}
