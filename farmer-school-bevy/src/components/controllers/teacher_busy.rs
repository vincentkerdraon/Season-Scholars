use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::{
    overlord::{Station, Teacher},
    teacher::*,
};

use super::moves::possible_move;

#[derive(Resource, Default)]
pub struct TeacherBusy {
    teachers_position: HashMap<Teacher, Option<(Station, f64)>>,
}

impl TeacherBusy {
    pub fn new(
        teachers: &Vec<(Teacher, Station, ActionShortDuration, ActionLongDuration)>,
    ) -> Self {
        let mut me = Self {
            teachers_position: HashMap::new(),
        };
        for (t, s, _, _) in teachers.iter() {
            me.teachers_position.insert(*t, Some((*s, 0.)));
        }
        me
    }

    pub fn moved(&mut self, e: &TeacherMovedEvent) {
        self.teachers_position
            .insert(e.teacher, Some((e.station_to, 0.)));
    }

    pub fn action(&mut self, t: Teacher, now: f64, duration: f64) -> bool {
        if let Some(Some((_, until))) = self.teachers_position.get_mut(&t) {
            *until = now + duration;
            return true;
        }
        false
    }

    /// returns (present_at_station, not_busy)
    pub fn ready(&mut self, t: Teacher, now: f64) -> (bool, bool) {
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

    pub fn is_station(&mut self, t: Teacher, station: &Station) -> bool {
        if let Some(Some((s, _))) = self.teachers_position.get(&t) {
            return station == s;
        }
        false
    }

    pub fn possible_move(&mut self, t: Teacher, from: Station, direction: Vec2) -> Option<Station> {
        if let Some((station_pointed, station_pointed_next)) = possible_move(from, direction) {
            let teacher_other = match t {
                Teacher::A => Teacher::B,
                Teacher::B => Teacher::A,
            };
            if let Some(Some((station_teacher_other, _))) =
                self.teachers_position.get(&teacher_other)
            {
                if *station_teacher_other != station_pointed {
                    return Some(station_pointed);
                }
                return Some(station_pointed_next);
            }
            return Some(station_pointed);
        }
        None
    }
}
