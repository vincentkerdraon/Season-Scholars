use crate::model::definitions::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct MoveTeacherEvent {
    pub station_from: Station,
    pub station_to: Station,
    pub teacher: Teacher,
}

#[derive(Event)]
pub struct TeacherMovedEvent {
    pub station_from: Station,
    pub station_to: Station,
    pub teacher: Teacher,
}
