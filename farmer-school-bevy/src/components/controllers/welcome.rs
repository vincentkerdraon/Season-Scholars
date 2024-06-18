use super::moves::possible_move;
use super::teacher_busy::TeacherBusy;
use crate::model::config::Config;

use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;
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

fn listen_reset(data: ResMut<WelcomeData>, mut reset_game_events: EventReader<ResetGameEvent>) {
    if reset_game_events.read().last().is_some() {
        reset(data);
    }
}

fn reset(mut data: ResMut<WelcomeData>) {
    data.teacher_busy = TeacherBusy::new(vec![STATION]);
    data.activated = true;
}

fn listen_game_over(
    mut data: ResMut<WelcomeData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.activated = false;
}

fn listen_graduated(
    mut data: ResMut<WelcomeData>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
) {
    for e in graduated_events.read() {
        data.students_classroom_nb = e.students.len() as i8;

        if data.students_classroom_nb <= 0 && !data.student_available {
            data.student_available = true;
            let emit = WelcomeAvailableEvent {};
            debug!("{:?}", emit);
            welcome_available_events.send(emit);
        }
    }
}

fn listen_monster_fed(
    config: Res<Config>,
    mut data: ResMut<WelcomeData>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
) {
    let students_max = config.students_rows_nb * 3;
    for e in monster_fed_events.read() {
        if e.needs.is_none() && data.students_classroom_nb < students_max && !data.student_available
        {
            data.student_available = true;
            let emit = WelcomeAvailableEvent {};
            debug!("{:?}", emit);
            welcome_available_events.send(emit);
        }
    }
}

fn listen_events(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<WelcomeData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
    mut welcome_student_events: EventWriter<WelcomeStudentEvent>,
    mut student_welcomed_events: EventWriter<StudentWelcomedEvent>,
    mut recruit_student_events: EventWriter<RecruitStudentEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    if !data.activated {
        return;
    }

    let students_max = config.students_rows_nb * 3;
    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        //ignore event if teacher is not at this station or if busy
        if data.teacher_busy.ready(e.teacher, now) != (true, true) {
            continue;
        }

        if e.long_action {
            if !data.student_available {
                data.teacher_busy
                    .action(e.teacher, now, config.long_action_s);
                let emit = RecruitStudentEvent { teacher: e.teacher };
                debug!("{:?}", emit);
                recruit_student_events.send(emit);

                data.student_available = true;
                let emit = WelcomeAvailableEvent {};
                debug!("{:?}", emit);
                welcome_available_events.send(emit);
            } else {
                let emit = InvalidActionStationEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
            continue;
        }

        if e.short_action {
            if data.student_available && data.students_classroom_nb < students_max {
                data.student_available = false;
                data.students_classroom_nb += 1;

                data.teacher_busy
                    .action(e.teacher, now, config.short_action_s);
                let emit = WelcomeStudentEvent {
                    teacher: Teacher::A,
                };
                debug!("{:?}", emit);
                welcome_student_events.send(emit);
                let emit = StudentWelcomedEvent { teacher: e.teacher };
                debug!("{:?}", emit);
                student_welcomed_events.send(emit);
            } else {
                let emit = InvalidActionStationEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
            continue;
        }

        if e.direction != Vec2::ZERO {
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
                trace!("{:?}", emit);
                invalid_move_events.send(emit);
            }
            continue;
        }
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
            .add_systems(Startup, reset)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_monster_fed)
            .add_systems(PreUpdate, listen_graduated)
            .add_systems(PreUpdate, listen_moved)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events);
    }
}

#[derive(Resource, Default)]
struct WelcomeData {
    students_classroom_nb: i8,
    student_available: bool,
    teacher_busy: TeacherBusy,
    activated: bool,
}
