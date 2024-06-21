use crate::model::config::Config;

use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::season::*;
use crate::model::students::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::process;

fn _debug_start_game(
    mut data: ResMut<Overlord>,
    mut reset_game_events: EventWriter<ResetGameEvent>,
    mut display_screen_game_events: EventWriter<DisplayScreenGameEvent>,
) {
    warn!("start game, no menu");
    data.screen = Screen::Game;
    let emit = DisplayScreenGameEvent {
        teachers: vec![Teacher::B],
    };
    debug!("{:?}", emit);
    display_screen_game_events.send(emit);

    data.screen = Screen::Game;
    let emit = ResetGameEvent {
        teachers: vec![Teacher::B],
    };
    debug!("{:?}", emit);
    reset_game_events.send(emit);
}

fn start(
    mut data: ResMut<Overlord>,
    mut display_screen_menu_events: EventWriter<DisplayScreenMenuEvent>,
) {
    info!("start game, display menu");
    data.screen = Screen::Menu;
    let emit = DisplayScreenMenuEvent {
        teachers: Vec::new(),
    };
    debug!("{:?}", emit);
    display_screen_menu_events.send(emit);
}

fn listen_events_score(
    mut data: ResMut<Overlord>,
    mut taught_events: EventReader<TaughtEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
) {
    for _ in taught_events.read() {
        data.score += 1;
    }
    for _ in graduated_events.read() {
        data.score += 10;
    }
    for e in monster_fed_events.read() {
        if e.needs.is_none() {
            data.score += 30;
        }
    }
    for e in season_changed_events.read() {
        data.seasons_elapsed = e.seasons_elapsed;
    }
}

fn listen_events_menu(
    time: Res<Time>,
    mut data: ResMut<Overlord>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut display_screen_game_events: EventWriter<DisplayScreenGameEvent>,
    mut display_screen_menu_events: EventWriter<DisplayScreenMenuEvent>,
    mut reset_game_events: EventWriter<ResetGameEvent>,
) {
    if data.screen != Screen::Menu {
        player_input_events.clear();
        return;
    }

    let mut changed = false;

    for e in player_input_events.read() {
        if e.short_action {
            if let std::collections::hash_map::Entry::Vacant(e) = data.teachers.entry(e.teacher) {
                e.insert(false);
                changed = true;
            } else {
                data.teachers.remove(&e.teacher);
                changed = true;
            }
        }

        if e.long_action && !data.teachers.is_empty() {
            let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
            let emit = DisplayScreenGameEvent {
                teachers: teachers.clone(),
            };
            debug!("{:?}", emit);
            display_screen_game_events.send(emit);

            data.game_started_s = time.elapsed_seconds_f64();
            data.score = 0;
            data.screen = Screen::Game;

            let emit = ResetGameEvent {
                teachers: teachers.clone(),
            };
            debug!("{:?}", emit);
            reset_game_events.send(emit);
        }
    }

    if changed {
        let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
        let emit = DisplayScreenMenuEvent { teachers };
        debug!("{:?}", emit);
        display_screen_menu_events.send(emit);
    }
}

fn listen_events_game_over(
    time: Res<Time>,
    mut data: ResMut<Overlord>,
    mut game_over_events: EventReader<GameOverEvent>,
    mut display_screen_game_over_recap_events: EventWriter<DisplayScreenGameOverRecapEvent>,
) {
    let now = time.elapsed_seconds_f64();
    if let Some(e) = game_over_events.read().last() {
        data.screen = Screen::GameOverRecap;
        let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
        let emit = DisplayScreenGameOverRecapEvent {
            teachers,
            reason: e.reason.to_string(),
            score: data.score,
            seasons_elapsed: data.seasons_elapsed,
            time_since_start_s: now - data.game_started_s,
        };
        debug!("{:?}", emit);
        display_screen_game_over_recap_events.send(emit);
    }
}

fn listen_events_reset(
    time: Res<Time>,
    mut data: ResMut<Overlord>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut display_screen_menu_events: EventWriter<DisplayScreenMenuEvent>,
    mut display_screen_game_over_recap_events: EventWriter<DisplayScreenGameOverRecapEvent>,
) {
    for e in player_input_events.read() {
        if !e.reset {
            continue;
        }
        let now = time.elapsed_seconds_f64();
        //debounce reset button
        if now - data.last_reset_s < 1.0 {
            continue;
        }
        data.last_reset_s = now;

        match data.screen {
            Screen::Game => {
                data.screen = Screen::GameOverRecap;
                let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
                let emit = DisplayScreenGameOverRecapEvent {
                    teachers,
                    reason: "Reset button".to_string(),
                    score: data.score,
                    seasons_elapsed: data.seasons_elapsed,
                    time_since_start_s: now - data.game_started_s,
                };
                debug!("{:?}", emit);
                display_screen_game_over_recap_events.send(emit);
            }
            Screen::GameOverRecap => {
                data.screen = Screen::Menu;
                let emit = DisplayScreenMenuEvent {
                    teachers: Vec::new(),
                };
                debug!("{:?}", emit);
                display_screen_menu_events.send(emit);
            }
            Screen::Menu => {
                warn!("press reset on menu => exit");
                process::exit(0x0100);
            }
        }
    }
}

pub struct OverlordControllerPlugin;

impl Plugin for OverlordControllerPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .insert_resource(Overlord {
                screen: Screen::Menu,
                last_reset_s: 0.,
                game_started_s: 0.,
                score: 0,
                seasons_elapsed: 0,
                teachers: HashMap::new(),
            })
            .add_event::<GameOverEvent>()
            .add_event::<ResetGameEvent>()
            .add_event::<DisplayScreenGameOverRecapEvent>()
            .add_event::<DisplayScreenGameEvent>()
            .add_event::<DisplayScreenMenuEvent>()
            .add_event::<InvalidActionStationEvent>()
            .add_event::<InvalidMoveEvent>()
            .add_systems(PreUpdate, listen_events_reset)
            .add_systems(PreUpdate, listen_events_game_over)
            .add_systems(PreUpdate, listen_events_score)
            .add_systems(PreUpdate, listen_events_menu);

        let mut debug = false;
        if let Some(config) = app.world.get_resource::<Config>() {
            if config.debug_start_game_immediately {
                //override normal start() for easy testing
                app.add_systems(Startup, _debug_start_game);
                debug = true;
            }
        }
        if !debug {
            app.add_systems(Startup, start);
        }
    }
}

#[derive(Resource)]
struct Overlord {
    screen: Screen,
    score: i32,
    last_reset_s: f64,
    teachers: HashMap<Teacher, bool>,
    game_started_s: f64,
    seasons_elapsed: i64,
}
