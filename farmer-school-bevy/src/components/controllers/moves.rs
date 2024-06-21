use bevy::prelude::*;

use crate::model::overlord::Station;

pub fn possible_move(from: Station, direction: Vec2) -> Option<(Station, Station)> {
    match from {
        Station::Welcome => match direction {
            Vec2 { x: -1.0, y: 0.0 } => Some((Station::Portal, Station::StudentLeft)),
            Vec2 { x: 0.0, y: -1.0 } => Some((Station::Portal, Station::StudentLeft)),
            Vec2 { x: 1.0, y: 0.0 } => Some((Station::Kitchen, Station::StudentRight)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::Kitchen => match direction {
            Vec2 { x: 0.0, y: -1.0 } => Some((Station::StudentRight, Station::StudentCenter)),
            Vec2 { x: -1.0, y: 0.0 } => Some((Station::Welcome, Station::Portal)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::Portal => match direction {
            Vec2 { x: 0.0, y: -1.0 } => Some((Station::StudentLeft, Station::StudentCenter)),
            Vec2 { x: 0.0, y: 1.0 } => Some((Station::Welcome, Station::Kitchen)),
            Vec2 { x: 1.0, y: 0.0 } => Some((Station::Welcome, Station::Kitchen)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentLeft => match direction {
            Vec2 { x: 0.0, y: 1.0 } => Some((Station::Portal, Station::Welcome)),
            Vec2 { x: 1.0, y: 0.0 } => Some((Station::StudentCenter, Station::StudentRight)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentCenter => match direction {
            Vec2 { x: 1.0, y: 0.0 } => Some((Station::StudentRight, Station::Kitchen)),
            Vec2 { x: -1.0, y: 0.0 } => Some((Station::StudentLeft, Station::Portal)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::StudentRight => match direction {
            Vec2 { x: 0.0, y: 1.0 } => Some((Station::Kitchen, Station::Welcome)),
            Vec2 { x: -1.0, y: 0.0 } => Some((Station::StudentCenter, Station::StudentLeft)),
            Vec2 { x: _, y: _ } => None,
        },
        Station::None => panic!(),
    }
}
