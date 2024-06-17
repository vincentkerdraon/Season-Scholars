use std::time::Duration;

use crate::components::moves::moves::possible_move;
use crate::components::teacher_busy::teacher_busy::TeacherBusy;
use crate::model::config::Config;
use crate::model::definitions::*;
use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::season::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;
use crate::model::{config::Config, definitions::*};
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;

impl SeasonTimer {
    pub fn new(seasons_duration: f64) -> Self {
        let mut s = Self {
            seasons_duration: seasons_duration,
            current_season: Season::Spring,
            seasons_elapsed: 0,
            timer: Timer::new(
                Duration::from_secs_f64(seasons_duration),
                TimerMode::Repeating,
            ),
        };
        s.timer.pause();
        s
    }

    fn next_season(&mut self) {
        self.current_season = match self.current_season {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        };
        self.seasons_elapsed += 1;
    }

    fn reset(&mut self) {
        self.current_season = Season::Spring;
        self.seasons_elapsed = 0;
        self.timer = Timer::new(
            Duration::from_secs_f64(self.seasons_duration),
            TimerMode::Repeating,
        );
    }

    fn tick(&mut self, s: Duration) -> bool {
        self.timer.tick(s);
        return self.timer.finished();
    }

    fn stop(&mut self) {
        self.timer.pause();
    }
}

pub fn season_startup_system(mut commands: Commands, config: Res<Config>) {
    debug!("season_startup_system starting");
    commands.spawn(SeasonTimer::new(config.seasons_duration_s));
}

fn listen_reset(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut reset_game_events: EventReader<ResetGameEvent>,
) {
    if reset_game_events.read().last().is_none() {
        return;
    }
    reset_game_events.clear();

    for (_entity, mut season_timer) in q.iter_mut() {
        season_timer.reset();
    }
}

fn season_timer_system(
    time: Res<Time>,
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut season_changed_events: EventWriter<SeasonChangedEvent>,
) {
    for (_entity, mut season_timer) in q.iter_mut() {
        if season_timer.tick(time.delta()) {
            season_timer.next_season();
            let e = SeasonChangedEvent {
                season: season_timer.current_season,
                seasons_elapsed: season_timer.seasons_elapsed,
            };
            debug!("{:?}", e);
            season_changed_events.send(e);
        }
    }
}

fn listen_game_over(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    for (_entity, mut season_timer) in q.iter_mut() {
        season_timer.stop();
    }
}

pub struct SeasonControllerPlugin;

impl Plugin for SeasonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, season_startup_system)
            .add_systems(PreUpdate, season_timer_system)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_event::<SeasonChangedEvent>();
    }
}

#[derive(Component)]
struct SeasonTimer {
    seasons_duration: f64,
    timer: Timer,
    current_season: Season,
    seasons_elapsed: i64,
}
