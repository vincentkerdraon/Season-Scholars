use super::events::*;
use crate::{
    components::{
        controllers::{
            overlord::events::{InvalidActionStationEvent, InvalidMoveEvent, ResetGameEvent},
            player_input::events::PlayerInputEvent,
            portal::events::MonsterFedEvent,
            students::events::GraduatedEvent,
            teacher::events::{MoveTeacherEvent, TeacherMovedEvent},
        },
        moves::moves::possible_move,
        teacher_busy::teacher_busy::TeacherBusy,
    },
    model::{
        config::Config,
        definitions::{Station, Teacher},
    },
};
use bevy::prelude::*;

const STATION: Station = Station::Welcome;

fn listen_moved(
    mut data: ResMut<WelcomeData>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
) {
    for e in teacher_moved_events.read() {
        data.teacher_busy.moved(e);
    }
}

fn listen_reset(mut data: ResMut<WelcomeData>, mut reset_game_events: EventReader<ResetGameEvent>) {
    if reset_game_events.read().last().is_some() {
        data.teacher_busy = TeacherBusy::new(vec![STATION]);
    }
}

fn listen_events(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<WelcomeData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
    mut welcome_student_events: EventWriter<WelcomeStudentEvent>,
    mut recruit_student_events: EventWriter<RecruitStudentEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    for e in graduated_events.read() {
        data.students_classroom_nb = e.students.len();
    }

    for _ in student_welcomed_events.read() {
        data.student_available = false;
        data.students_classroom_nb += 1;
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
        if data.teacher_busy.ready(e.teacher, now) != (true, true) {
            continue;
        }

        if e.long_action {
            if data.student_available {
                let emit = InvalidActionStationEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            } else {
                data.teacher_busy
                    .action(e.teacher, now, config.long_action_s);
                let emit = RecruitStudentEvent { teacher: e.teacher };
                debug!("{:?}", emit);
                recruit_student_events.send(emit);

                should_accept = true
            }
        }

        if e.short_action {
            if data.student_available {
                data.teacher_busy
                    .action(e.teacher, now, config.short_action_s);
                let emit = WelcomeStudentEvent {
                    teacher: Teacher::A,
                };
                debug!("{:?}", emit);
                welcome_student_events.send(emit);
            } else {
                let emit = InvalidActionStationEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
        }

        if e.confirm_move {
            if let Some(to) = possible_move(STATION, e.direction) {
                let emit = MoveTeacherEvent {
                    station_from: STATION,
                    station_to: to,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                move_teacher_events.send(emit);
            } else {
                let emit = InvalidMoveEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_move_events.send(emit);
            }
        }
    }

    if data.students_classroom_nb == config.students_max {
        return;
    }

    if data.students_classroom_nb == 0 {
        should_accept = true;
    }

    if should_accept && !data.student_available {
        data.student_available = true;
        let emit = WelcomeAvailableEvent {};
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
            .add_event::<RecruitStudentEvent>()
            .insert_resource(WelcomeData { ..default() })
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_moved)
            .add_systems(PreUpdate, listen_events);
    }
}

#[derive(Resource, Default)]
struct WelcomeData {
    students_classroom_nb: usize,
    student_available: bool,
    teacher_busy: TeacherBusy,
}
