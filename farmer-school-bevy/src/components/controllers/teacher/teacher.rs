use bevy::prelude::*;

use crate::{
    components::controllers::overlord::events::ResetGameEvent,
    model::definitions::{Station, Teacher},
};

use super::events::*;

pub struct TeacherControllerPlugin;

impl Plugin for TeacherControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveTeacherEvent>()
            .add_event::<TeacherMovedEvent>()
            .add_systems(PreUpdate, listen_reset);
    }
}

fn listen_reset(
    mut reset_game_events: EventReader<ResetGameEvent>,
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
) {
    if let Some(e) = reset_game_events.read().last() {
        if e.teachers.contains(&Teacher::A) {
            let emit = TeacherMovedEvent {
                teacher: Teacher::A,
                station_from: Station::StudentLeft,
                station_to: Station::StudentCenter,
            };
            debug!("{:?}", emit);
            teacher_moved_events.send(emit);
        }
        if e.teachers.contains(&Teacher::B) {
            let emit = TeacherMovedEvent {
                teacher: Teacher::B,
                station_from: Station::Portal,
                station_to: Station::Welcome,
            };
            debug!("{:?}", emit);
            teacher_moved_events.send(emit);
        }
    }
}
