use std::time::Duration;

use crate::{
    components::controllers::welcome::events::{StudentWelcomedEvent, WelcomeAvailableEvent},
    model::definitions::{Season, StudentCols, Teacher},
    ResetGameEvent,
};
use bevy::prelude::*;

use super::events::SeasonChangedEvent;

#[derive(Component)]
pub struct SeasonTimer {
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
}

pub fn season_startup_system(mut commands: Commands) {
    debug!("season_startup_system starting");
    commands.spawn(SeasonTimer {
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
            season_timer.timer.reset()
        }
    }
}

pub fn season_timer_system(
    time: Res<Time>,
    mut q: Query<(Entity, &mut SeasonTimer)>,
    mut season_changed_events: EventWriter<SeasonChangedEvent>,

    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
    mut student_welcomed_events: EventWriter<StudentWelcomedEvent>,
) {
    for (_entity, mut season_timer) in q.iter_mut() {
        season_timer.timer.tick(time.delta());
        if season_timer.timer.finished() {
            season_timer.next_season();
            let e = SeasonChangedEvent {
                season: season_timer.current_season,
                seasons_elapsed: season_timer.seasons_elapsed,
            };
            debug!("{:?}", e);
            season_changed_events.send(e);

            //FIXME DEBUG
            if season_timer.seasons_elapsed % 2 == 0 {
                let e = WelcomeAvailableEvent { available: true };
                debug!("DEBUG ONLY {:?}", e);
                welcome_available_events.send(e);
            } else {
                let e = StudentWelcomedEvent {
                    student_id: "aa".to_string(),
                    student_pos_col: StudentCols::Left,
                    student_pos_row: 1,
                    teacher: Teacher::A,
                };
                debug!("DEBUG ONLY {:?}", e);
                student_welcomed_events.send(e);
            }
        }
    }
}

pub struct SeasonPlugin;

impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, season_startup_system)
            .add_systems(Startup, season_system)
            .add_systems(PreUpdate, season_timer_system)
            .add_event::<SeasonChangedEvent>();
    }
}
