use bevy::prelude::*;

use crate::{
    components::controllers::welcome::events::{StudentWelcomedEvent, WelcomeAvailableEvent},
    config::Config,
};

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<WelcomeResources>,
) {
    data.closed = asset_server.load(config.clone().base_path + "Welcome/DoorClosed.png");
    data.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller1.png"));
    data.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller2.png"));
    data.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller3.png"));

    data.door = commands
        .spawn(SpriteBundle {
            texture: data.closed.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 398.0,
                    y: 412.0,
                    z: 1.0,
                },
                scale: Vec3 {
                    x: 0.54,
                    y: 0.54,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        })
        .id();
}

pub fn listen_events(
    mut welcome_available_events: EventReader<WelcomeAvailableEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut data: ResMut<WelcomeResources>,
    mut query: Query<&mut Handle<Image>>,
) {
    for _ in student_welcomed_events.read() {
        if let Ok(mut texture_handle) = query.get_mut(data.door) {
            *texture_handle = data.get_closed().clone();
        }
    }

    let entity = data.door;
    for _ in welcome_available_events.read() {
        if let Some(texture_new) = data.get_opened_auto() {
            if let Ok(mut texture_handle) = query.get_mut(entity) {
                *texture_handle = texture_new.clone();
            }
        }
    }
}

pub struct WelcomeViewPlugin;

impl Plugin for WelcomeViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_events)
            .insert_resource(WelcomeResources::new())
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
pub struct WelcomeResources {
    door: Entity,
    closed: Handle<Image>,
    opened: Vec<Handle<Image>>,
    opened_last_used_index: usize,
}

impl WelcomeResources {
    pub fn new() -> Self {
        Self {
            door: Entity::PLACEHOLDER,
            opened: Vec::new(),
            closed: Handle::default(),
            opened_last_used_index: 0,
        }
    }

    pub fn get_closed(&self) -> &Handle<Image> {
        &self.closed
    }
    pub fn get_opened_auto(&mut self) -> Option<&Handle<Image>> {
        self.opened_last_used_index = self.opened_last_used_index + 1;
        if self.opened_last_used_index == self.opened.len() {
            self.opened_last_used_index = 0;
        }
        self.opened.get(self.opened_last_used_index)
    }
    pub fn _get_opened(&self, index: usize) -> Option<&Handle<Image>> {
        self.opened.get(index)
    }
}
