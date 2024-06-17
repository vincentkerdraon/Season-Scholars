use bevy::prelude::*;
use strum_macros::EnumIter;

use super::{
    overlord::{Station, Teacher},
    season::Season,
};

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

pub type StudentId = i64;
pub type StudentRow = i8;

#[derive(Debug, Default, Clone, Hash)]
pub struct Student {
    pub id: StudentId,
    pub row: StudentRow,
    pub col: StudentCol,
    pub knowledge: Vec<Season>,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StudentCol {
    #[default]
    Left = 1,
    Center,
    Right,
}

pub fn station_to_student_col(station: Station) -> StudentCol {
    match station {
        Station::StudentLeft => StudentCol::Left,
        Station::StudentCenter => StudentCol::Center,
        Station::StudentRight => StudentCol::Right,
        _ => panic!(),
    }
}
