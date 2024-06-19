use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::season::*;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<RoomData>,
) {
    let spring = asset_server.load(
        config
            .base_path
            .join("images/ready/Classroom/ClassroomSpring.png"),
    );

    data.seasons.insert(Season::Spring, spring.clone());
    data.seasons.insert(
        Season::Summer,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/ClassroomSummer.png"),
        ),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/ClassroomAutumn.png"),
        ),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/ClassroomWinter.png"),
        ),
    );

    let sprite = SpriteBundle {
        texture: spring.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: -30.,
                z: -1.,
            },
            scale: Vec3 {
                x: 0.73,
                y: 0.73,
                z: 1.,
            },
            ..default()
        },
        ..default()
    };
    let sprite_entity = commands.spawn(sprite).id();
    data.entity = sprite_entity;
}

fn listen_events(
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    data: Res<RoomData>,
    mut query: Query<&mut Handle<Image>>,
) {
    for e in season_changed_events.read() {
        if let Ok(mut texture_handle) = query.get_mut(data.entity) {
            if let Some(h) = data.seasons.get(&e.season) {
                *texture_handle = h.clone();
            }
        }
    }
}

pub struct RoomViewPlugin;

impl Plugin for RoomViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoomData::new())
            .add_systems(Startup, load_resources)
            .add_systems(Update, listen_events);
    }
}

#[derive(Resource)]
struct RoomData {
    entity: Entity,
    seasons: HashMap<Season, Handle<Image>>,
}

impl RoomData {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            seasons: HashMap::new(),
        }
    }
}
