use std::collections::HashMap;

use bevy::prelude::*;

use crate::{components::controllers::teacher::events::TeacherMovedEvent, model::definitions::*};

#[derive(Resource, Default)]
pub struct TeacherBusy {
    here: Vec<Station>,
    teachers: HashMap<Teacher, Option<(Station, f64)>>,
}

impl TeacherBusy {
    pub fn new(here: Vec<Station>) -> Self {
        Self {
            here: here,
            teachers: HashMap::new(),
        }
    }
    pub fn moved(&mut self, e: &TeacherMovedEvent) {
        if self.here.len() == 0 {
            panic!();
        }
        if !self.here.contains(&e.station_from) {
            self.teachers.remove(&e.teacher);
        }
        if self.here.contains(&e.station_to) {
            self.teachers.insert(e.teacher, Some((e.station_to, 0.)));
        }
    }

    pub fn action(&mut self, t: Teacher, now: f64, duration: f64) -> bool {
        if self.here.len() == 0 {
            panic!();
        }
        if let Some(d) = self.teachers.get_mut(&t) {
            if let Some((_, until)) = d {
                *until = now + duration;
                return true;
            }
        }
        false
    }

    pub fn ready(&mut self, t: Teacher, now: f64) -> (bool, bool) {
        if self.here.len() == 0 {
            panic!();
        }
        if let Some(d) = self.teachers.get(&t) {
            if let Some((_, until)) = d {
                if *until < now {
                    return (true, false);
                }
                return (true, true);
            } else {
                panic!()
            }
        }
        return (false, false);
    }

    pub fn station(&mut self, t: Teacher) -> Option<Station> {
        if let Some(d) = self.teachers.get(&t) {
            if let Some((s, _)) = d {
                return Some(*s);
            }
        }
        return None;
    }
}
