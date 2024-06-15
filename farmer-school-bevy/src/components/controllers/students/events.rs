use crate::model::definitions::*;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct GraduateEvent {
    pub teacher: Teacher,
    pub student_col: StudentCol,
}

/// The first student of the col exits the classroom and will fulfill the monsters need with what he learned
#[derive(Event, Debug)]
pub struct GraduatedEvent {
    pub teacher: Teacher,
    pub knowledge: Vec<Season>,
    pub student_id: StudentId,
    pub students: Vec<Student>,
}

#[derive(Event, Debug)]
pub struct TeachEvent {
    pub student_col: StudentCol,
    pub teacher: Teacher,
}

/// All students in the column learn the season
#[derive(Event, Debug)]
pub struct TaughtEvent {
    pub student_col: StudentCol,
    pub teacher: Teacher,
    pub knowledge: Season,
    pub students: Vec<Student>,
}

#[derive(Event, Debug)]
pub struct StudentsSeatedEvent {
    pub students: Vec<Student>,
}
