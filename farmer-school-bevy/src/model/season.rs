use bevy::prelude::*;
use rand::Rng;
use strum_macros::EnumIter;

/// After some time, the season switched to the next one
#[derive(Event, Debug)]
pub struct SeasonChangedEvent {
    pub season: Season,
    pub seasons_elapsed: i64,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Spring = 1,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let s = rng.gen_range(1..=4);
        s.into()
    }
}

impl From<usize> for Season {
    fn from(num: usize) -> Self {
        match num {
            1 => Season::Spring,
            2 => Season::Summer,
            3 => Season::Autumn,
            4 => Season::Winter,
            _ => unreachable!(),
        }
    }
}
