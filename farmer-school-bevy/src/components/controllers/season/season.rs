use std::time::Duration;

use crate::{
    components::controllers::{
        overlord::events::{GameOverEvent, ResetGameEvent},
        welcome::events::{StudentWelcomedEvent, WelcomeAvailableEvent},
    },
    config::Config,
    model::definitions::{Season, StudentCols, Teacher},
};
use bevy::prelude::*;

use super::events::SeasonChangedEvent;

#[derive(Component)]
pub struct SeasonTimer {
    seasons_duration: f64,
    timer: Timer,
    current_season: Season,
    seasons_elapsed: i32,
}

impl SeasonTimer {
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
    commands.spawn(SeasonTimer {
        seasons_duration: config.clone().seasons_duration_s,
        current_season: Season::Spring,
        seasons_elapsed: 0,
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    });
}

pub fn season_system(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut reset_game_events: EventReader<ResetGameEvent>,
) {
    for _ in reset_game_events.read() {
        for (_, mut season_timer) in q.iter_mut() {
            debug!("season_timer reset");
            season_timer.reset();
        }
    }
}

pub fn season_timer_system(
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

pub fn listen_game_over(
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

pub fn listen_reset(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut reset_game_events: EventReader<ResetGameEvent>,
) {
    if reset_game_events.read().last().is_none() {
        return;
    }
    for (_entity, mut season_timer) in q.iter_mut() {
        season_timer.reset();
    }
}

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, season_startup_system)
            .add_systems(Startup, season_system)
            .add_systems(PreUpdate, season_timer_system)
            .add_systems(PreUpdate, listen_game_over)
            .add_event::<SeasonChangedEvent>();
    }
}
