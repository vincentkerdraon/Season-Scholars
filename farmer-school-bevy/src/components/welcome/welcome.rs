// src/components/welcome.rs

use super::events::*;
use crate::{
    components::teacher::events::TeacherMovedEvent,
    model::{definitions::Teacher, events::*},
};
use bevy::prelude::*;

pub struct Welcome;

pub fn welcome_system(
    mut commands: Commands,
    mut welcome_events: EventReader<TeacherMovedEvent>,
    mut player_input_events: EventReader<PlayerInput>,
    mut welcome_available_events: EventWriter<WelcomeAvailableEvent>,
    mut welcome_student_events: EventWriter<WelcomeStudentEvent>,
    mut student_welcomed_events: EventWriter<StudentWelcomedEvent>,
) {
    info!("welcome_system starting");
    for _ in welcome_events.read() {
        info!("read welcome welcome_events");
        // Process TeacherMovedEvent
        // Emit WelcomeAvailableEvent
        welcome_available_events.send(WelcomeAvailableEvent { is_available: true });
        student_welcomed_events.send(StudentWelcomedEvent {
            student_id: "a".to_string(),
            student_pos_col: crate::model::definitions::StudentCols::Center,
            student_pos_row: 1,
            teacher: Teacher::A,
        });
    }

    for _ in player_input_events.read() {
        info!("read welcome player_input_events");
        // Process PlayerInput
        // Emit WelcomeStudentEvent
        welcome_student_events.send(WelcomeStudentEvent {
            teacher: Teacher::A,
        });
    }

    // Process other events and emit necessary events
}

pub struct WelcomePlugin;

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WelcomeAvailableEvent>()
            .add_event::<WelcomeStudentEvent>()
            .add_event::<StudentWelcomedEvent>()
            .add_systems(Startup, welcome_system);
    }
}
