// src/components/teacher.rs

use crate::{
    components::player_input::events::PlayerInputEvent,
    model::{definitions::*, events::*},
};
use bevy::prelude::*;

use super::events::*;

pub struct TeacherPlugin;

pub fn teacher_system(
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut teacher_moved_events: EventWriter<TeacherMovedEvent>,
) {
    info!("teacher_system starting");
    for event in player_input_events.read() {
        info!("read teacher player_input_events");
        teacher_moved_events.send(TeacherMovedEvent {
            station_from: Station::None,
            station_to: Station::StudentLeft,
            teacher: Teacher::A,
        });
    }
}

impl Plugin for TeacherPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveTeacherEvent>()
            .add_event::<TeacherMovedEvent>()
            .add_systems(Startup, teacher_system);
    }
}
