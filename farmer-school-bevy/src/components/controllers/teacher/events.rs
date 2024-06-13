use crate::model::definitions::*;
use bevy::prelude::*;

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
