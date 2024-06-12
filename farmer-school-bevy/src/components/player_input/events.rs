use crate::model::definitions::*;
use bevy::prelude::*;

/// A player action, like pointing to the left and pressing the Long action button
#[derive(Event, Debug)]
pub struct PlayerInputEvent {
    /// x=1 => right; y=1 => top
    pub direction: Vec2,
    pub long_action: bool,
    pub short_action: bool,
    pub teacher: Teacher,
}
