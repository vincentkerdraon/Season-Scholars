use crate::model::definitions::*;
use bevy::prelude::*;

/// A teacher is gathering information on the next monster needs
#[derive(Event, Debug)]
pub struct ObservePortalEvent {
    pub teacher: Teacher,
}

/// Show information on the monsters needs (current or in line)
#[derive(Event, Debug)]
pub struct PortalObservedEvent {
    pub teacher: Teacher,
    pub monsters: Vec<Monster>,
    pub health: i8,
}

/// Show information on the monsters needs (current or in line)
#[derive(Event, Debug)]
pub struct MonsterPoppedEvent {
    pub monsters: Vec<Monster>,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct Monster {
    ///time in seconds when revealed
    pub next_wait_s: f64,
    ///duration in seconds between attacks
    pub attack_interval_s: f64,
    ///time in seconds when attacking
    pub next_attack_s: f64,
    pub needs: Vec<Season>,
    pub revealed: bool,
}

/// The monster in the portal attacked the portal
#[derive(Event, Debug)]
pub struct PortalAttackedEvent {
    pub health: i8,
    pub monsters: Vec<Monster>,
}

/// The current monster at the portal has some needs fulfilled
#[derive(Event, Debug)]
pub struct MonsterFedEvent {
    pub teacher: Teacher,
    pub needs: Option<Vec<Season>>,
}
