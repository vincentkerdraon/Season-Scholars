use super::teacher_busy::TeacherBusy;
use crate::components::controllers::teacher_tired::TeacherTired;
use crate::model::config::Config;
use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::season::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;
use bevy::prelude::*;

const STATION: Station = Station::Kitchen;

fn listen_game_over(
    mut data: ResMut<KitchenData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.component_ready = ComponentReady {
        listen_data_events: false,
        listen_player_input: false,
    };
}

fn listen_reset(
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
    mut reset_game_step1_events: EventReader<ResetGameStep1Event>,
    mut reset_game_step2_events: EventReader<ResetGameStep2Event>,
    mut reset_game_step3_events: EventReader<ResetGameStep3Event>,
    mut students_eat_events: EventWriter<StudentsEatEvent>,
) {
    if let Some(e) = reset_game_step1_events.read().last() {
        data.component_ready.listen_data_events = true;
        data.food_remaining = config.food_max;
        data.teacher_busy = TeacherBusy::new(&e.teachers);
        data.teacher_tired = TeacherTired::new(&e.teachers);
    }
    if let Some(_e) = reset_game_step2_events.read().last() {
        let emit = StudentsEatEvent {
            food_remaining: data.food_remaining,
        };
        debug!("{:?}", emit);
        students_eat_events.send(emit);
    }
    if let Some(_e) = reset_game_step3_events.read().last() {
        data.component_ready.listen_player_input = true;
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
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut students_eat_events: EventWriter<StudentsEatEvent>,
) {
    if config.debug_disable_student_eating {
        return;
    }
    for e in season_changed_events.read() {
        if e.season != Season::Winter {
            continue;
        }
        data.food_remaining -= 1;

        let emit = StudentsEatEvent {
            food_remaining: data.food_remaining,
        };
        debug!("{:?}", emit);
        students_eat_events.send(emit);

        if data.food_remaining < 0 {
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

fn listen_events_teacher_tired(
    time: Res<Time>,
    mut data: ResMut<KitchenData>,
    mut teacher_tired_events: EventReader<TeacherTiredEvent>,
) {
    for e in teacher_tired_events.read() {
        let now = time.elapsed_seconds_f64();
        data.teacher_tired
            .update(now, &e.teacher, e.short_action, e.long_action)
    }
}

fn listen_events_player_input(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut teacher_eat_events: EventWriter<TeacherAteEvent>,
    mut cook_events: EventWriter<CookedEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    if !data.component_ready.listen_player_input {
        player_input_events.clear();
        return;
    }

    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        if !data.teacher_busy.is_station(e.teacher, &STATION) {
            continue;
        }
        if data.teacher_busy.ready(e.teacher, now) != (true, true) {
            continue;
        }

        if e.long_action {
            if data.food_remaining < config.food_max {
                let (_, long) = data.teacher_tired.get(&e.teacher).unwrap();
                data.teacher_busy.action(e.teacher, now, long);
                data.food_remaining = config.food_max;
                let emit = CookedEvent {
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
                trace!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
            continue;
        }

        if e.short_action {
            if data.food_remaining > 0 {
                let now = time.elapsed_seconds_f64();
                data.food_remaining -= 1;
                let (short, _) = data.teacher_tired.get(&e.teacher).unwrap();
                data.teacher_busy.action(e.teacher, now, short);

                let emit = TeacherAteEvent {
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
                trace!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
            continue;
        }

        if e.direction != Vec2::ZERO {
            if let Some(to) = data
                .teacher_busy
                .possible_move(e.teacher, STATION, e.direction)
            {
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

pub struct KitchenControllerPlugin;

impl Plugin for KitchenControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KitchenData { ..default() })
            .add_event::<CookedEvent>()
            .add_event::<StudentsEatEvent>()
            .add_event::<TeacherAteEvent>()
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_teacher_tired)
            .add_systems(PreUpdate, listen_season)
            .add_systems(PreUpdate, listen_moved)
            .add_systems(PreUpdate, listen_students)
            .add_systems(PreUpdate, listen_events_player_input);
    }
}

#[derive(Resource, Default)]
struct KitchenData {
    component_ready: ComponentReady,
    food_remaining: i8,
    teacher_busy: TeacherBusy,
    teacher_tired: TeacherTired,
    students_classroom_nb: i8,
}
