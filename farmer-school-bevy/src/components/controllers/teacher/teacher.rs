use bevy::prelude::*;

use crate::components::moves::moves::possible_move;
use crate::components::teacher_busy::teacher_busy::TeacherBusy;
use crate::model::config::Config;
use crate::model::definitions::*;
use crate::model::definitions::{Station, Teacher};
use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::season::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;
use crate::model::{config::Config, definitions::*};
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;

pub struct TeacherControllerPlugin;

impl Plugin for TeacherControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveTeacherEvent>()
            .add_event::<TeacherMovedEvent>()
            .add_systems(PreUpdate, listen_move)
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

fn listen_move(
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
    mut move_teacher_events: EventReader<MoveTeacherEvent>,
) {
    for e in move_teacher_events.read() {
        let emit = TeacherMovedEvent {
            teacher: e.teacher,
            station_from: e.station_from,
            station_to: e.station_to,
        };
        debug!("{:?}", emit);
        teacher_moved_events.send(emit);
    }
}
