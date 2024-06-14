use crate::model::definitions::*;
use bevy::prelude::*;

/// After some time, the season switched to the next one
#[derive(Event, Debug)]
pub struct SeasonChangedEvent {
    pub season: Season,
    pub seasons_elapsed: i64,
}
