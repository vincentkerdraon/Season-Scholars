use bevy::prelude::*;

use super::overlord::Teacher;

/// A new student is waiting
#[derive(Event, Debug)]
pub struct WelcomeAvailableEvent {}

/// Action of accepting a new student at the door
#[derive(Event, Debug)]
pub struct WelcomeStudentEvent {
    pub teacher: Teacher,
}

#[derive(Event, Debug)]
pub struct StudentWelcomedEvent {
    pub teacher: Teacher,
}
/// Action of fetching a student when not available
#[derive(Event, Debug)]
pub struct RecruitStudentEvent {
    pub teacher: Teacher,
}
