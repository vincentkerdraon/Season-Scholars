use std::time::Duration;

use crate::model::config::Config;

use crate::model::overlord::*;
use crate::model::season::*;
use bevy::prelude::*;

impl SeasonTimer {
    pub fn new(seasons_duration: f64) -> Self {
        let mut s = Self {
            seasons_duration,
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
        self.timer.finished()
    }

    fn stop(&mut self) {
        self.timer.pause();
    }
}

pub fn season_startup_system(mut commands: Commands, config: Res<Config>) {
    commands.spawn(SeasonTimer::new(config.seasons_duration_s));
}

fn listen_reset(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    // mut reset_game_step1_events: EventReader<ResetGameStep1Event>,
    mut reset_game_step2_events: EventReader<ResetGameStep2Event>,
) {
    if let Some(_e) = reset_game_step2_events.read().last() {
        for (_entity, mut season_timer) in q.iter_mut() {
            season_timer.reset();
        }
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
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, season_timer_system)
            .add_systems(PreUpdate, listen_reset)
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
