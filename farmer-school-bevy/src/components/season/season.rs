use std::time::Duration;

use crate::{model::definitions::Season, ResetGameEvent};
use bevy::prelude::*;

use super::events::SeasonChangedEvent;

#[derive(Component)]
pub struct SeasonTimer {
    timer: Timer,
    current_season: Season,
    seasons_elapsed: i32,
}

impl SeasonTimer {
    pub fn new(duration: f32, start_season: Season) -> Self {
        SeasonTimer {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            current_season: start_season,
            seasons_elapsed: 0,
        }
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
}

pub fn season_startup_system(mut commands: Commands) {
    info!("season_startup_system starting");
    commands.spawn((SeasonTimer {
        current_season: Season::Spring,
        seasons_elapsed: 0,
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    },));
}

pub fn season_system(
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut reset_game_events: EventReader<ResetGameEvent>,
) {
    for event in reset_game_events.read() {
        for (entity, mut season_timer) in q.iter_mut() {
            info!("season_timer reset");
            season_timer.timer.reset()
        }
    }
}

pub fn season_timer_system(
    time: Res<Time>,
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut season_changed_events: EventWriter<SeasonChangedEvent>,
) {
    for (entity, mut season_timer) in q.iter_mut() {
        season_timer.timer.tick(time.delta());
        if season_timer.timer.finished() {
            info!("season_timer finished");
            season_timer.next_season();
            season_changed_events.send(SeasonChangedEvent {
                season: season_timer.current_season,
                seasons_elapsed: season_timer.seasons_elapsed,
            });
        }
    }
}

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, season_startup_system)
            .add_systems(Startup, season_system)
            .add_systems(Update, season_timer_system)
            .add_event::<SeasonChangedEvent>();
    }
}
