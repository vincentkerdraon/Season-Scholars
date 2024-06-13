use crate::model::definitions::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct GraduateEvent {
    pub teacher: Teacher,
    pub student_col: StudentCols,
}

/// The first student of the col exits the classroom and will fulfill the monsters need with what he learned
#[derive(Event, Debug)]
pub struct GraduatedEvent {
    pub teacher: Teacher,
    pub knowledge: Vec<Season>,
    pub student_id: String,
}

#[derive(Event, Debug)]
pub struct TeachEvent {
    pub station: Station,
    pub teacher: Teacher,
}

/// All students in the column learn the season
#[derive(Event, Debug)]
pub struct TaughtEvent {
    pub station: Station,
    pub teacher: Teacher,
    pub knowledge: Season,
}
