// src/components/teacher.rs

use crate::model::{
    definitions::{Direction, *},
    events::*,
};
use bevy::prelude::*;

use super::events::*;

pub struct TeacherPlugin;

pub fn teacher_system(
    mut player_input_events: EventReader<PlayerInput>,
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
) {
    info!("teacher_system starting");
    for event in player_input_events.read() {
        info!("read teacher player_input_events");
        // Process PlayerInput
        // Emit TeacherMovedEvent based on the direction
        if event.direction == Direction::Left {
            teacher_moved_events.send(TeacherMovedEvent {
                station_from: Station::None,
                station_to: Station::StudentLeft,
                teacher: Teacher::A,
            });
        } else if event.direction == Direction::Right {
            teacher_moved_events.send(TeacherMovedEvent {
                station_from: Station::None,
                station_to: Station::StudentRight,
                teacher: Teacher::A,
            });
        }
    }
}

impl Plugin for TeacherPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveTeacherEvent>()
            .add_event::<TeacherMovedEvent>()
            .add_systems(Startup, teacher_system);
    }
}