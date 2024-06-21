use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::{
    overlord::{Station, Teacher},
    teacher::*,
};

#[derive(Resource, Default)]
pub struct TeacherBusy {
    here: Vec<Station>,
    teachers_position: HashMap<Teacher, Option<(Station, f64)>>,
}

impl TeacherBusy {
    pub fn new(here: Vec<Station>) -> Self {
        Self {
            here,
            teachers_position: HashMap::new(),
        }
    }
    pub fn moved(&mut self, e: &TeacherMovedEvent) {
        if self.here.is_empty() {
            //FIXME start new game with player2 => immediate panic here
            panic!();
        }
        if self.here.contains(&e.station_from) {
            self.teachers_position.remove(&e.teacher);
        }
        if self.here.contains(&e.station_to) {
            self.teachers_position
                .insert(e.teacher, Some((e.station_to, 0.)));
        }
    }

    pub fn action(&mut self, t: Teacher, now: f64, duration: f64) -> bool {
        if self.here.is_empty() {
            panic!();
        }
        if let Some(Some((_, until))) = self.teachers_position.get_mut(&t) {
            *until = now + duration;
            return true;
        }

        false
    }

    /// returns (present_at_station, not_busy)
    pub fn ready(&mut self, t: Teacher, now: f64) -> (bool, bool) {
        if self.here.is_empty() {
            panic!();
        }
        if let Some(d) = self.teachers_position.get(&t) {
            if let Some((_, until)) = d {
                if *until > now {
                    return (true, false);
                }
                return (true, true);
            } else {
                panic!()
            }
        }
        (false, false)
    }

    pub fn station(&mut self, t: Teacher) -> Option<Station> {
        if let Some(Some((s, _))) = self.teachers_position.get(&t) {
            return Some(*s);
        }
        None
    }
}
