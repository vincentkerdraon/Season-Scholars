use crate::model::definitions::*;
use bevy::prelude::*;


/// A new student is waiting and available to welcome (or not)
#[derive(Event)]
pub struct WelcomeAvailableEvent {
    pub is_available: bool,
}

/// Action of accepting a new student at the door
#[derive(Event)]
pub struct WelcomeStudentEvent{
    pub teacher: Teacher,
}

/// Action of accepting a new student at the door
#[derive(Event)]
pub struct StudentWelcomedEvent {
    pub teacher: Teacher,
    pub student_pos_col: StudentCols,
    pub student_pos_row: i32,
    pub student_id: String,
}
