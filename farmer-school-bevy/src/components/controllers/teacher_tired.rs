use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::{
    overlord::{Station, Teacher},
    teacher::*,
};

#[derive(Resource, Default)]
pub struct TeacherTired {
    //(last updated, short, long)
    teachers_speed: HashMap<Teacher, (f64, ActionShortDuration, ActionLongDuration)>,
}

impl TeacherTired {
    pub fn new(
        teachers: &Vec<(Teacher, Station, ActionShortDuration, ActionLongDuration)>,
    ) -> Self {
        let mut me = Self {
            teachers_speed: HashMap::new(),
        };
        for (t, _, short, long) in teachers.iter() {
            me.teachers_speed.insert(*t, (0., *short, *long));
        }
        me
    }

    pub fn is_slower(
        &mut self,
        now: f64,
        t: &Teacher,
        delta: f64,
        increment: f64,
        short_max: ActionShortDuration,
        long_max: ActionLongDuration,
    ) -> Option<(bool, ActionShortDuration, ActionLongDuration)> {
        if let Some((update, short, long)) = self.teachers_speed.get_mut(t) {
            if *update + delta < now {
                *update = now;
                *short += increment;
                if *short > short_max {
                    *short = short_max;
                }
                *long += increment;
                if *long > long_max {
                    *long = long_max;
                }
                return Some((true, *short, *long));
            }
            return Some((false, *short, *long));
        }
        None
    }

    pub fn update(
        &mut self,
        now: f64,
        t: &Teacher,
        action_short: ActionShortDuration,
        action_long: ActionLongDuration,
    ) {
        self.teachers_speed
            .insert(*t, (now, action_short, action_long));
    }

    pub fn get(&mut self, t: &Teacher) -> Option<(ActionShortDuration, ActionLongDuration)> {
        if let Some((_, short, long)) = self.teachers_speed.get(t) {
            return Some((*short, *long));
        }
        None
    }
}
