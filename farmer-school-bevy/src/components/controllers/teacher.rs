use std::num::Wrapping;

use bevy::prelude::*;

use crate::model::config::Config;
use crate::model::kitchen::TeacherAteEvent;
use crate::model::overlord::*;
use crate::model::teacher::*;

use super::teacher_tired::TeacherTired;

fn listen_game_over(
    mut data: ResMut<TeacherData>,
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
    time: Res<Time>,
    mut data: ResMut<TeacherData>,
    mut reset_game_step1_events: EventReader<ResetGameStep1Event>,
    mut reset_game_step2_events: EventReader<ResetGameStep2Event>,
    mut reset_game_step3_events: EventReader<ResetGameStep3Event>,
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
) {
    if let Some(e) = reset_game_step1_events.read().last() {
        data.component_ready.listen_data_events = true;
        data.teachers = Vec::new();
        let now = time.elapsed_seconds_f64();

        for (t, _, short, long) in e.teachers.iter() {
            data.teachers.push(*t);
            data.teacher_tired.update(now, t, *short, *long);
        }
    }
    if let Some(e) = reset_game_step2_events.read().last() {
        //initial station of teachers
        for (t, s, _, _) in e.teachers.iter() {
            let emit = TeacherMovedEvent {
                teacher: *t,
                station_from: Station::StudentLeft,
                station_to: *s,
            };
            debug!("{:?}", emit);
            teacher_moved_events.send(emit);
        }
    }
    if let Some(_e) = reset_game_step3_events.read().last() {
        data.component_ready.listen_player_input = true;
    }
}

fn update_teacher_speed(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<TeacherData>,
    mut teacher_tired_events: EventWriter<TeacherTiredEvent>,
) {
    if !data.component_ready.listen_data_events {
        return;
    }

    data.frame += Wrapping(1);
    if !data.frame.0 % config.draw_frame_modulo != 0 {
        return;
    }

    let now = time.elapsed_seconds_f64();
    for t in data.teachers.clone() {
        if let Some((updated, short, long)) = data.teacher_tired.is_slower(
            now,
            &t,
            config.actions_increase_delta,
            config.actions_increase_increment,
            config.short_action_s_max,
            config.long_action_s_max,
        ) {
            if updated {
                let emit = TeacherTiredEvent {
                    teacher: t,
                    short_action: short,
                    long_action: long,
                };
                debug!("{:?}", emit);
                teacher_tired_events.send(emit);
            }
        }
    }
}

fn listen_events_teacher_ate(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<TeacherData>,
    mut teacher_eat_events: EventReader<TeacherAteEvent>,
    mut teacher_tired_events: EventWriter<TeacherTiredEvent>,
) {
    if !data.component_ready.listen_data_events {
        teacher_eat_events.clear();
        return;
    }

    for e in teacher_eat_events.read() {
        let now = time.elapsed_seconds_f64();
        data.teacher_tired.update(
            now,
            &e.teacher,
            config.long_action_s_min,
            config.short_action_s_min,
        );
        let emit = TeacherTiredEvent {
            teacher: e.teacher,
            long_action: config.long_action_s_min,
            short_action: config.short_action_s_min,
        };
        debug!("{:?}", emit);
        teacher_tired_events.send(emit);
    }
}

fn listen_move(
    mut move_teacher_events: EventReader<MoveTeacherEvent>,
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
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

pub struct TeacherControllerPlugin;

impl Plugin for TeacherControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TeacherData { ..default() })
            .add_event::<MoveTeacherEvent>()
            .add_event::<TeacherMovedEvent>()
            .add_event::<TeacherTiredEvent>()
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, update_teacher_speed)
            .add_systems(PreUpdate, listen_events_teacher_ate)
            .add_systems(PreUpdate, listen_move);
    }
}

#[derive(Resource, Default)]
struct TeacherData {
    component_ready: ComponentReady,
    teacher_tired: TeacherTired,
    teachers: Vec<Teacher>,
    frame: Wrapping<i8>,
}
