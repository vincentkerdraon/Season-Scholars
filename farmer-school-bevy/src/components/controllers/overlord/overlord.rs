use super::events::*;
use crate::components::controllers::portal::events::MonsterFedEvent;
use crate::{
    components::controllers::player_input::events::PlayerInputEvent, model::definitions::*,
};
use crate::{GraduatedEvent, TaughtEvent};
use bevy::prelude::*;
use std::collections::HashMap;
use std::process;

#[derive(Resource)]
pub struct Overlord {
    screen: Screen,
    score: i32,
    last_reset_s: f64,
    teachers: HashMap<Teacher, bool>,
    game_started_s: f64,
}

pub struct OverlordPlugin;

impl Plugin for OverlordPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Overlord {
            screen: Screen::Menu,
            last_reset_s: 0.,
            game_started_s: 0.,
            score: 0,
            teachers: HashMap::new(),
        })
        .add_event::<GameOverEvent>()
        .add_event::<ResetGameEvent>()
        .add_event::<DisplayScreenGameOverRecapEvent>()
        .add_event::<DisplayScreenGameEvent>()
        .add_event::<DisplayScreenMenuEvent>()
        .add_event::<InvalidActionStationEvent>()
        .add_event::<InvalidMoveEvent>()
        .add_systems(Startup, start)
        .add_systems(PreUpdate, listen_events_game_over)
        .add_systems(PreUpdate, listen_events_reset)
        .add_systems(PreUpdate, listen_events_score)
        .add_systems(PreUpdate, listen_events_menu);
    }
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
) {
    for _ in taught_events.read() {
        data.score = data.score + 1;
    }
    for _ in graduated_events.read() {
        data.score = data.score + 10;
    }
    for e in monster_fed_events.read() {
        if e.needs.is_none() {
            data.score = data.score + 30;
        }
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
            if data.teachers.get(&e.teacher).is_some() {
                data.teachers.remove(&e.teacher);
                changed = true;
            } else {
                data.teachers.insert(e.teacher, false);
                changed = true;
            }
        }

        if e.long_action && data.teachers.len() > 0 {
            let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
            let emit = DisplayScreenGameEvent { teachers: teachers };
            debug!("{:?}", emit);
            display_screen_game_events.send(emit);

            data.game_started_s = time.elapsed_seconds_f64();
            data.score = 0;
            data.screen = Screen::Game;

            let emit = ResetGameEvent {};
            debug!("{:?}", emit);
            reset_game_events.send(emit);
        }
    }

    if changed {
        let teachers: Vec<Teacher> = data.teachers.keys().copied().collect();
        let emit = DisplayScreenMenuEvent { teachers: teachers };
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
    for e in game_over_events.read() {
        data.screen = Screen::GameOverRecap;
        let emit = DisplayScreenGameOverRecapEvent {
            reason: e.reason.to_string(),
            score: data.score,
            time_since_start_s: now - data.game_started_s,
        };
        debug!("{:?}", emit);
        display_screen_game_over_recap_events.send(emit);
        return;
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
                let emit = DisplayScreenGameOverRecapEvent {
                    reason: "".to_string(),
                    score: data.score,
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
