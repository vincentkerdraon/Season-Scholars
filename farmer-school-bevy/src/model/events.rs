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

/// A teacher is gathering information on the next monster needs
#[derive(Event, Debug)]
pub struct ObservePortal {
    pub teacher: Teacher,
}

/// Show information on the monsters needs (current or in line)
#[derive(Event, Debug)]
pub struct PortalObserved {
    pub teacher: Teacher,
    pub is_open: bool,
    pub window_id: i32,
    pub needs: Vec<Season>,
}

#[derive(Event, Debug)]
pub struct PortalAttackedEvent {
    pub remaining_life: i32,
}
/// The monster in the portal attacked the portal

/// The current monster at the portal has some needs fulfilled
#[derive(Event, Debug)]
pub struct MonsterFedEvent {
    pub is_open: bool,
    pub needs: Vec<Season>,
}

///Start from zero
#[derive(Event, Debug)]
pub struct ResetGameEvent {}

#[derive(Event, Debug)]
pub struct GameOverEvent {
    pub score: i32,
    pub reason: String,
    pub time_since_start_ms: i64,
}

/// Functional error because the requested action is not possible
#[derive(Event, Debug)]
pub struct InvalidActionStation {
    pub station: Station,
    pub teacher: Teacher,
}
