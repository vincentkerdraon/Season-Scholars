// src/components/Portal.rs

use super::events::*;
use crate::{
    components::controllers::{
        overlord::events::{
            GameOverEvent, InvalidActionStationEvent, InvalidMoveEvent, ResetGameEvent,
        },
        player_input::events::PlayerInputEvent,
        season::events::SeasonChangedEvent,
        teacher::events::{MoveTeacherEvent, TeacherMovedEvent},
    },
    config::Config,
    model::{
        definitions::{Season, Teacher},
        events::*,
    },
};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Portal {
    activated: bool,
    monsters: Vec<Monster>,
    difficulty: i32,
    health: i8,
    health_max: i8,
    teachers_present: HashMap<Teacher, Option<f64>>,
}

pub fn listen_game_over(
    mut data: ResMut<Portal>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }

    data.activated = false;
}

pub fn monster_attack(
    time: Res<Time>,
    mut data: ResMut<Portal>,
    mut portal_attacked_events: EventWriter<PortalAttackedEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    if !data.activated {
        return;
    }

    if let Some(monster) = data.monsters.first_mut() {
        let mut changed = false;
        let now = time.elapsed_seconds_f64();
        if monster.next_wait_s < now && !monster.revealed {
            monster.revealed = true;
            changed = true;
        }

        if monster.revealed && monster.next_attack_s < now {
            monster.next_attack_s = now + monster.attack_interval_s;
            data.health = data.health - 1;
            changed = true;
        }

        if changed {
            let emit = PortalAttackedEvent {
                health: data.health,
                monsters: data.monsters.clone(),
            };
            debug!("{:?}", emit);
            portal_attacked_events.send(emit);
        }

        if data.health <= 0 {
            let emit = GameOverEvent {
                reason: "Portal destroyed".to_string(),
            };
            debug!("{:?}", emit);
            game_over_events.send(emit);

            //stop processing this func until reset
            data.activated = false;
        }
    }
}

pub fn listen_events(
    time: Res<Time>,
    mut data: ResMut<Portal>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
    mut monster_fed_events: EventWriter<MonsterFedEvent>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    let now = time.elapsed_seconds_f64();

    for e in graduated_events.read() {
        //Remove seasons 1 for 1.
        //Example:  needs=<Spring, Spring, Winter> ; knowledge = <Spring, Winter> => need=<Spring>
        let monster = data.monsters.first_mut().unwrap();
        for k in &e.knowledge {
            if let Some(pos) = monster.needs.iter().position(|&x| x == *k) {
                monster.needs.remove(pos);
            }
        }
        if monster.needs.len() == 0 {
            data.monsters.remove(0);
        }

        let mut emit = MonsterFedEvent {
            needs: None,
            teacher: e.teacher,
        };
        let first = data.monsters.first();
        if let Some(monster) = first {
            emit.needs = Some(monster.needs.clone());
        }
        debug!("{:?}", emit);
        monster_fed_events.send(emit);

        //there must be always at least one monster
        if data.monsters.len() == 0 {
            pop_monster(now, &mut data, &mut monster_popped_events)
        }
    }

    for e in teacher_moved_events.read() {
        if e.station_to == crate::model::definitions::Station::Portal {
            data.teachers_present.insert(e.teacher, None);
        }
        if e.station_from == crate::model::definitions::Station::Portal {
            data.teachers_present.remove(&e.teacher);
        }
    }
}

pub fn listen_reset(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<Portal>,
    mut reset_game_events: EventReader<ResetGameEvent>,
    monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    if reset_game_events.read().next().is_some() {
        data.activated = true;
        reset(time, config, data, monster_popped_events);
        reset_game_events.clear();
    }
}

pub fn reset(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<Portal>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    data.health_max = config.clone().portal_health_max;
    data.health = data.health_max;
    data.difficulty = 0;
    data.teachers_present = HashMap::new();
    let now = time.elapsed_seconds_f64();
    data.monsters = Vec::new();
    pop_monster(now, &mut data, &mut monster_popped_events);
}

pub fn listen_events_create_monster(
    time: Res<Time>,
    mut data: ResMut<Portal>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    let now = time.elapsed_seconds_f64();
    for _ in season_changed_events.read() {
        pop_monster(now, &mut data, &mut monster_popped_events);
    }
}

fn pop_monster(
    now: f64,
    data: &mut Portal,
    monster_popped_events: &mut EventWriter<MonsterPoppedEvent>,
) {
    let m = generate_monster(now, data.difficulty);
    data.monsters.push(m);
    data.difficulty = data.difficulty + 1;

    let emit = MonsterPoppedEvent {
        monsters: data.monsters.clone(),
    };
    debug!("difficulty {}, {:?}", data.difficulty, emit);
    monster_popped_events.send(emit);
}

fn generate_monster(now: f64, difficulty: i32) -> Monster {
    let mut m = Monster { ..default() };
    match difficulty {
        1 => {
            m.next_wait_s = now + 40.;
            m.attack_interval_s = 60.;
            m.needs = vec![Season::Autumn];
        }
        2 => {
            m.next_wait_s = now + 40.;
            m.attack_interval_s = 60.;
            m.needs = vec![Season::Spring];
        }
        3 => {
            m.next_wait_s = now + 40.;
            m.attack_interval_s = 60.;
            m.needs = vec![Season::Summer, Season::Winter];
        }
        4 => {
            m.next_wait_s = now + 40.;
            m.attack_interval_s = 60.;
            m.needs = vec![Season::Winter];
        }
        5 => {
            m.next_wait_s = now + 30.;
            m.attack_interval_s = 60.;
            m.needs = vec![Season::Autumn, Season::Winter, Season::Spring];
        }
        6..=9 => {
            m.next_wait_s = now + 30.;
            m.attack_interval_s = 60.;
            m.needs = random_needs(1, 2);
        }
        10..=14 => {
            m.next_wait_s = now + 20.;
            m.attack_interval_s = 30.;
            m.needs = random_needs(1, 3);
        }
        15..=19 => {
            m.next_wait_s = now + 15.;
            m.attack_interval_s = 30.;
            m.needs = random_needs(1, 3);
        }
        20..=24 => {
            m.next_wait_s = now + 15.;
            m.attack_interval_s = 30.;
            m.needs = random_needs(2, 3);
        }
        25..=29 => {
            m.next_wait_s = now + 10.;
            m.attack_interval_s = 30.;
            m.needs = random_needs(2, 3);
        }
        30..=34 => {
            m.next_wait_s = now + 10.;
            m.attack_interval_s = 30.;
            m.needs = random_needs(3, 3);
        }
        35..=49 => {
            m.next_wait_s = now + 10.;
            m.attack_interval_s = 10.;
            m.needs = random_needs(3, 3);
        }
        _ if difficulty >= 50 => {
            m.next_wait_s = now + 5.;
            m.attack_interval_s = 10.;
            m.needs = random_needs(3, 3);
        }
        _ => {
            m.next_wait_s = now + 5.;
            m.attack_interval_s = 10.;
        }
    }
    m.next_attack_s = m.next_wait_s + m.attack_interval_s;
    m
}

fn random_needs(min: i8, max: i8) -> Vec<Season> {
    let mut res = Vec::new();
    let mut rng = rand::thread_rng();

    // Generate a random number of elements in the range [min, max]
    let n = rng.gen_range(min..=max);

    // Add random seasons to the result
    for _ in 0..n {
        let s = rng.gen_range(1..=4);
        let season = match s {
            1 => Season::Spring,
            2 => Season::Summer,
            3 => Season::Autumn,
            4 => Season::Winter,
            _ => unreachable!(),
        };
        res.push(season);
    }
    res
}

pub fn listen_events_player_input(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<Portal>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut portal_observed_events: EventWriter<PortalObservedEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
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

        //FIXME, it is technically possible to do a short + long + move at the same time.

        if e.long_action {
            if data.health >= data.health_max {
                let emit = InvalidActionStationEvent {
                    station: crate::model::definitions::Station::Portal,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            } else {
                data.teachers_present
                    .insert(e.teacher, Some(now + config.long_action_s));
                //repair //FIXME event
                data.health = data.health + 1;
            }
        }

        if e.short_action {
            let mut revealed = false;
            for monster in data.monsters.iter_mut() {
                if !monster.revealed {
                    revealed = true;
                    monster.revealed = true;

                    data.teachers_present
                        .insert(e.teacher, Some(now + config.short_action_s));

                    let emit = PortalObservedEvent {
                        teacher: Teacher::A,
                        health: data.health,
                        monsters: data.monsters.clone(),
                    };
                    debug!("{:?}", emit);
                    portal_observed_events.send(emit);
                    break;
                }
            }
            if !revealed {
                let emit = InvalidActionStationEvent {
                    station: crate::model::definitions::Station::Portal,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
        }

        if e.confirm_move {
            match e.direction {
                Vec2 { x: 1.0, y: 1.0 } => {
                    let emit = MoveTeacherEvent {
                        station_from: crate::model::definitions::Station::Portal,
                        station_to: crate::model::definitions::Station::Welcome,
                        teacher: e.teacher,
                    };
                    debug!("{:?}", emit);
                    move_teacher_events.send(emit);
                }
                Vec2 { x: 1.0, y: -1.0 } => {
                    let emit = MoveTeacherEvent {
                        station_from: crate::model::definitions::Station::Portal,
                        station_to: crate::model::definitions::Station::StudentLeft,
                        teacher: e.teacher,
                    };
                    debug!("{:?}", emit);
                    move_teacher_events.send(emit);
                }
                Vec2 { x: _, y: _ } => {
                    let emit = InvalidMoveEvent {
                        station: crate::model::definitions::Station::Portal,
                        teacher: e.teacher,
                    };
                    debug!("{:?}", emit);
                    invalid_move_events.send(emit);
                }
            }
        }
    }
}

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObservePortalEvent>()
            .add_event::<PortalObservedEvent>()
            .add_event::<PortalAttackedEvent>()
            .add_event::<MonsterFedEvent>()
            .add_event::<MonsterPoppedEvent>()
            .insert_resource(Portal { ..default() })
            .add_systems(Startup, reset)
            .add_systems(PreUpdate, listen_events)
            .add_systems(PreUpdate, listen_events_create_monster)
            .add_systems(PreUpdate, monster_attack)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_player_input);
    }
}
