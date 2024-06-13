use bevy::prelude::*;

use crate::{
    components::controllers::welcome::events::{StudentWelcomedEvent, WelcomeAvailableEvent},
    config::Config,
};

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut res: ResMut<WelcomeResources>,
) {
    res.closed = asset_server.load(config.clone().base_path + "Welcome/DoorClosed.png");
    res.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller1.png"));
    res.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller2.png"));
    res.opened
        .push(asset_server.load(config.clone().base_path + "Welcome/Traveller3.png"));

    let sprite = SpriteBundle {
        texture: res.closed.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 400.0,
                y: 460.0,
                z: 1.0,
            },
            scale: Vec3 {
                x: 0.8,
                y: 0.8,
                z: 1.,
            },
            ..default()
        },
        ..default()
    };
    let sprite_entity = commands.spawn(sprite).id();
    res.entity = sprite_entity;
}

pub fn listen_events(
    mut welcome_available_events: EventReader<WelcomeAvailableEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut res: ResMut<WelcomeResources>,
    mut query: Query<&mut Handle<Image>>,
) {
    for _ in student_welcomed_events.read() {
        if let Ok(mut texture_handle) = query.get_mut(res.entity) {
            *texture_handle = res.get_closed().clone();
        }
    }

    let entity = res.entity;
    for _ in welcome_available_events.read() {
        if let Some(texture_new) = res.get_opened_auto() {
            if let Ok(mut texture_handle) = query.get_mut(entity) {
                *texture_handle = texture_new.clone();
            }
        }
    }
}

pub struct WelcomePlugin;

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_events)
            .insert_resource(WelcomeResources::new())
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
pub struct WelcomeResources {
    entity: Entity,
    closed: Handle<Image>,
    opened: Vec<Handle<Image>>,
    opened_last_used_index: usize,
}

impl WelcomeResources {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
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
