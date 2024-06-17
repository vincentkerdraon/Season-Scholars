use super::moves::possible_move;
use super::teacher_busy::TeacherBusy;
use crate::model::config::Config;

use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::season::*;
use crate::model::students::*;
use crate::model::teacher::*;
use bevy::prelude::*;
use rand::Rng;

const STATION: Station = Station::Portal;

fn listen_game_over(
    mut data: ResMut<PortalData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }

    data.activated = false;
}

fn monster_attack(
    time: Res<Time>,
    mut data: ResMut<PortalData>,
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
            data.health -= 1;
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

fn listen_events(
    time: Res<Time>,
    mut data: ResMut<PortalData>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut monster_fed_events: EventWriter<MonsterFedEvent>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    if !data.activated {
        return;
    }

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
        if monster.needs.is_empty() {
            data.monsters.remove(0);
        }

        let mut emit = MonsterFedEvent {
            needs: None,
            teacher: e.teacher,
            monsters: data.monsters.clone(),
        };
        let first = data.monsters.first();
        if let Some(monster) = first {
            emit.needs = Some(monster.needs.clone());
        }
        debug!("{:?}", emit);
        monster_fed_events.send(emit);

        //there must be always at least one monster
        if data.monsters.is_empty() {
            pop_monster(now, &mut data, &mut monster_popped_events)
        }
    }
}

fn listen_moved(
    mut data: ResMut<PortalData>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
) {
    for e in teacher_moved_events.read() {
        data.teacher_busy.moved(e);
    }
}

fn listen_reset(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut reset_game_events: EventReader<ResetGameEvent>,
    monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    if reset_game_events.read().last().is_some() {
        data.activated = true;
        reset(time, config, data, monster_popped_events);
        reset_game_events.clear();
    }
}

fn reset(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    data.health_max = config.portal_health_max;
    data.health = data.health_max;
    data.difficulty = 0;
    // data.teachers_present = HashMap::new();
    let now = time.elapsed_seconds_f64();
    data.monsters = Vec::new();
    data.teacher_busy = TeacherBusy::new(vec![STATION]);
    pop_monster(now, &mut data, &mut monster_popped_events);
}

fn listen_events_create_monster(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    mut monster_popped_events: EventWriter<MonsterPoppedEvent>,
) {
    if config.debug_disable_monster_attack {
        return;
    }
    let now = time.elapsed_seconds_f64();
    for _ in season_changed_events.read() {
        pop_monster(now, &mut data, &mut monster_popped_events);
    }
}

fn pop_monster(
    now: f64,
    data: &mut PortalData,
    monster_popped_events: &mut EventWriter<MonsterPoppedEvent>,
) {
    // ignore if already too many
    if data.monsters.len() > 6 {
        debug!("already too many monsters, skipping pop.");
        return;
    }

    //first should be difficulty=id=1
    data.difficulty += 1;
    let m = generate_monster(now, data.difficulty);
    data.monsters.push(m);

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
    m.id = difficulty;
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
        res.push(Season::rand());
    }
    res
}

fn listen_events_player_input(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut portal_observed_events: EventWriter<PortalObservedEvent>,
    mut observe_portal_events: EventWriter<ObservePortalEvent>,
    mut portal_fixed_events: EventWriter<PortalFixedEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    if !data.activated {
        return;
    }

    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        if data.teacher_busy.ready(e.teacher, now) != (true, true) {
            continue;
        }

        if e.long_action {
            if data.health >= data.health_max {
                let emit = InvalidActionStationEvent {
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            } else {
                data.teacher_busy
                    .action(e.teacher, now, config.long_action_s);
                data.health += 1;
                let emit = PortalFixedEvent {
                    teacher: Teacher::A,
                    health: data.health,
                    monsters: data.monsters.clone(),
                };
                debug!("{:?}", emit);
                portal_fixed_events.send(emit);
            }
            continue;
        }

        if e.short_action {
            let mut revealed = false;
            for monster in data.monsters.iter_mut() {
                if !monster.revealed {
                    revealed = true;
                    monster.revealed = true;

                    data.teacher_busy
                        .action(e.teacher, now, config.short_action_s);

                    let emit = ObservePortalEvent {
                        teacher: Teacher::A,
                    };
                    debug!("{:?}", emit);
                    observe_portal_events.send(emit);
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
                    station: STATION,
                    teacher: e.teacher,
                };
                debug!("{:?}", emit);
                invalid_action_station_events.send(emit);
            }
            continue;
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
            continue;
        }
    }
}

pub struct PortalControllerPlugin;

impl Plugin for PortalControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObservePortalEvent>()
            .add_event::<PortalObservedEvent>()
            .add_event::<PortalFixedEvent>()
            .add_event::<PortalAttackedEvent>()
            .add_event::<MonsterFedEvent>()
            .add_event::<MonsterPoppedEvent>()
            .insert_resource(PortalData { ..default() })
            .add_systems(Startup, reset)
            .add_systems(PreUpdate, listen_events)
            .add_systems(PreUpdate, listen_events_create_monster)
            .add_systems(PreUpdate, monster_attack)
            .add_systems(PreUpdate, listen_moved)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_player_input);
    }
}

#[derive(Resource, Default)]
struct PortalData {
    activated: bool,
    monsters: Vec<Monster>,
    difficulty: i32,
    health: i8,
    health_max: i8,
    teacher_busy: TeacherBusy,
}
