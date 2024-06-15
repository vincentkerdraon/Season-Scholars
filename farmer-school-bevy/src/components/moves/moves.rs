use bevy::prelude::*;

use crate::model::definitions::Station;

pub fn possible_move(from: Station, direction: Vec2) -> Option<Station> {
    match from {
        Station::Welcome => match direction {
            Vec2 { x: -1.0, y: -1.0 } => Some(Station::Portal),
            Vec2 { x: 1.0, y: 0.0 } => Some(Station::Kitchen),
            Vec2 { x: _, y: _ } => None,
        },
        Station::Kitchen => match direction {
            Vec2 { x: 0.0, y: -1.0 } => Some(Station::StudentRight),
            Vec2 { x: -1.0, y: 0.0 } => Some(Station::Welcome),
            Vec2 { x: _, y: _ } => None,
        },
        Station::Portal => match direction {
            Vec2 { x: 1.0, y: -1.0 } => Some(Station::StudentLeft),
            Vec2 { x: 1.0, y: 1.0 } => Some(Station::Welcome),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentLeft => match direction {
            Vec2 { x: 1.0, y: -1.0 } => Some(Station::Portal),
            Vec2 { x: 1.0, y: 0.0 } => Some(Station::StudentCenter),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentCenter => match direction {
            Vec2 { x: 1.0, y: 0.0 } => Some(Station::StudentRight),
            Vec2 { x: -1.0, y: 0.0 } => Some(Station::StudentLeft),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentRight => match direction {
            Vec2 { x: 0.0, y: 1.0 } => Some(Station::Kitchen),
            Vec2 { x: -1.0, y: 0.0 } => Some(Station::StudentCenter),
            Vec2 { x: _, y: _ } => None,
        },
        Station::None => panic!(),
    }
}
