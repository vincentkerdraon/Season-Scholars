use bevy::prelude::*;

use super::overlord::Teacher;

/// A player action, like pointing to the left and pressing the Long action button
#[derive(Event, Debug, Default)]
pub struct PlayerInputEvent {
    /// x=1 => right; y=1 => top
    pub direction: Vec2,
    pub confirm_move: bool,
    pub long_action: bool,
    pub short_action: bool,
    pub reset: bool,
    pub teacher: Teacher,
}
