use super::events::*;
use crate::components::controllers::welcome::events::StudentWelcomedEvent;
use crate::components::moves::moves::possible_move;
use crate::{
    components::{
        controllers::{
            overlord::events::{
                GameOverEvent, InvalidActionStationEvent, InvalidMoveEvent, ResetGameEvent,
            },
            player_input::events::PlayerInputEvent,
            season::events::SeasonChangedEvent,
            students::events::GraduatedEvent,
            teacher::events::{MoveTeacherEvent, TeacherMovedEvent},
        },
        teacher_busy::teacher_busy::TeacherBusy,
    },
    model::{config::Config, definitions::Station},
};
use bevy::prelude::*;

const STATION: Station = Station::Kitchen;

fn listen_game_over(
    mut data: ResMut<KitchenData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.activated = false;
}

fn listen_reset(
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
    mut reset_game_events: EventReader<ResetGameEvent>,
) {
    if reset_game_events.read().last().is_some() {
        data.activated = true;
        data.food_remaining = config.food_max;
        data.teacher_busy = TeacherBusy::new(vec![STATION]);
    }
}

fn listen_moved(
    mut data: ResMut<KitchenData>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
) {
    for e in teacher_moved_events.read() {
        data.teacher_busy.moved(e);
    }
}

fn listen_season(
    mut data: ResMut<KitchenData>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut students_eat_events: EventWriter<StudentsEatEvent>,
) {
    for _ in season_changed_events.read() {
        // data.food_remaining -= data.students_classroom_nb / 3;
        data.food_remaining -= 1;

        let emit = StudentsEatEvent {
            food_remaining: data.food_remaining,
        };
        debug!("{:?}", emit);
        students_eat_events.send(emit);

        if data.food_remaining <= 0 {
            let emit = GameOverEvent {
                reason: "Students starving".to_string(),
            };
            debug!("{:?}", emit);
            game_over_events.send(emit);
            return;
        }
    }
}

fn listen_students(
    mut data: ResMut<KitchenData>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
) {
    for e in graduated_events.read() {
        data.students_classroom_nb = e.students.len() as i8;
    }

    for _ in student_welcomed_events.read() {
        data.students_classroom_nb += 1;
    }
}

fn listen_events_player_input(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut teacher_eat_events: EventWriter<TeacherEatEvent>,
    mut cook_events: EventWriter<CookEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        if !data.activated {
            continue;
        }
        if data.teacher_busy.ready(e.teacher, now) != (true, true) {
            continue;
        }

        if e.long_action {
            if data.food_remaining < config.food_max {
                data.food_remaining = config.food_max;
                let emit = CookEvent {
                    food_remaining: data.food_remaining,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                cook_events.send(emit);
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
            if data.food_remaining > 0 {
                data.food_remaining -= 1;

                let emit = TeacherEatEvent {
                    food_remaining: data.food_remaining,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                teacher_eat_events.send(emit);
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

        if e.confirm_move {
            let from = STATION;
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
            continue;
        }
    }
}

pub struct KitchenControllerPlugin;

impl Plugin for KitchenControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KitchenData { ..default() })
            .add_event::<CookEvent>()
            .add_event::<StudentsEatEvent>()
            .add_event::<TeacherEatEvent>()
            .add_systems(PreUpdate, listen_moved)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_season)
            .add_systems(PreUpdate, listen_students)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_player_input);
    }
}

#[derive(Resource, Default)]
struct KitchenData {
    activated: bool,
    food_remaining: i8,
    teacher_busy: TeacherBusy,
    students_classroom_nb: i8,
}