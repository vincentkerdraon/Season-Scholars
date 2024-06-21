use bevy::prelude::*;

use super::{overlord::Teacher, season::Season};

/// A teacher is gathering information on the next monster needs
#[derive(Event, Debug)]
pub struct ObservePortalEvent {
    pub teacher: Teacher,
}

pub type PortalHealth = i8;

/// Show information on the monsters needs (current or in line)
#[derive(Event, Debug)]
pub struct PortalObservedEvent {
    pub teacher: Teacher,
    pub monsters: Vec<Monster>,
    pub health: PortalHealth,
}

/// Fix the portal
#[derive(Event, Debug)]
pub struct PortalFixedEvent {
    pub teacher: Teacher,
    pub monsters: Vec<Monster>,
    pub health: PortalHealth,
}

/// Show information on the monsters needs (current or in line)
#[derive(Event, Debug)]
pub struct MonsterPoppedEvent {
    pub monsters: Vec<Monster>,
    pub health: PortalHealth,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct Monster {
    pub id: i32,
    ///time in seconds when revealed
    pub next_wait_s: f64,
    ///duration in seconds between attacks
    pub attack_interval_s: f64,
    ///time in seconds when attacking
    pub next_attack_s: f64,
    pub needs: Vec<Season>,
    pub monster_visible: bool,
    pub window_revealed: bool,
}

/// The monster in the portal attacked the portal
#[derive(Event, Debug)]
pub struct PortalAttackedEvent {
    pub health: PortalHealth,
    pub monsters: Vec<Monster>,
}

/// The current monster at the portal has some needs fulfilled
#[derive(Event, Debug)]
pub struct MonsterFedEvent {
    pub teacher: Teacher,
    pub needs: Option<Vec<Season>>,
    pub monsters: Vec<Monster>,
}
