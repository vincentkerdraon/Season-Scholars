// src/components/welcome.rs

use super::events::*;
use crate::{
    components::{
        controllers::{
            overlord::events::{InvalidActionStationEvent, InvalidMoveEvent},
            player_input::events::PlayerInputEvent,
            portal::events::MonsterFedEvent,
            teacher::events::{MoveTeacherEvent, TeacherMovedEvent},
        },
        moves::moves::possible_move,
    },
    config::Config,
    model::{
        definitions::{Station, Teacher},
        events::*,
    },
};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Welcome {
    students_classroom_nb: usize,
    students_classroom_max: usize,
    available: bool,
    teachers_present: HashMap<Teacher, Option<f64>>,
}

pub fn init(config: Res<Config>, mut data: ResMut<Welcome>) {
    data.students_classroom_max = config.clone().students_max;
}

fn listen_events(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<Welcome>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
    mut welcome_student_events: EventWriter<WelcomeStudentEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    for _ in graduated_events.read() {
        data.students_classroom_nb = data.students_classroom_nb - 1;
    }

    for _ in student_welcomed_events.read() {
        data.available = false;
        data.students_classroom_nb = data.students_classroom_nb + 1;
    }

    for e in teacher_moved_events.read() {
        if e.station_to == crate::model::definitions::Station::Welcome {
            data.teachers_present.insert(e.teacher, None);
        }
        if e.station_from == crate::model::definitions::Station::Welcome {
            data.teachers_present.remove(&e.teacher);
        }
    }

    let mut should_accept = false;

    for e in monster_fed_events.read() {
        if e.needs == None {
            should_accept = true;
        }
    }

    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        //ignore event if teacher is not at this station or if busy
        let busy_until = match data.teachers_present.get(&e.teacher) {
            Some(busy_until) => *busy_until, // Extract the value from Some(free)
            None => continue,                // the teacher is not present
        };
        if let Some(busy_until) = busy_until {
            if now < busy_until {
                continue; // Skip if the teacher is not yet free
            }
        }

        if e.long_action {
            if data.available {
                let emit = InvalidActionStationEvent {
                    station: crate::model::definitions::Station::Welcome,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            } else {
                data.teachers_present
                    .insert(e.teacher, Some(now + config.long_action_s));
                //recruit //FIXME event
                should_accept = true
            }
        }

        if e.short_action {
            if data.available {
                data.teachers_present
                    .insert(e.teacher, Some(now + config.short_action_s));
                let emit = WelcomeStudentEvent {
                    teacher: Teacher::A,
                };
                debug!("{:?}", emit);
                welcome_student_events.send(emit);
            } else {
                let emit = InvalidActionStationEvent {
                    station: crate::model::definitions::Station::Welcome,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
        }

        if e.confirm_move {
            let from = Station::Welcome;
            if let Some(to) = possible_move(from, e.direction) {
                let emit = MoveTeacherEvent {
                    station_from: from,
                    station_to: to,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                move_teacher_events.send(emit);
            } else {
                let emit = InvalidMoveEvent {
                    station: from,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_move_events.send(emit);
            }
        }
    }

    if data.students_classroom_nb == data.students_classroom_max {
        return;
    }

    if data.students_classroom_nb == 0 {
        should_accept = true;
    }

    if should_accept && !data.available {
        data.available = true;
        let emit = WelcomeAvailableEvent { available: true };
        debug!("{:?}", emit);
        welcome_available_events.send(emit);
        return;
    }
}

pub struct WelcomeControllerPlugin;

impl Plugin for WelcomeControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WelcomeAvailableEvent>()
            .add_event::<WelcomeStudentEvent>()
            .add_event::<StudentWelcomedEvent>()
            .insert_resource(Welcome { ..default() })
            .add_systems(Startup, init)
            .add_systems(PreUpdate, listen_events);
    }
}
