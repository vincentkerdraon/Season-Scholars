use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::controllers::season::events::SeasonChangedEvent, config::Config,
    model::definitions::Season,
};

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut res: ResMut<RoomResources>,
) {
    let spring = asset_server.load(config.clone().base_path + "Classroom/ClassroomSpring.png");
    let spring2 = spring.clone();

    res.seasons.insert(Season::Autumn, spring);
    res.seasons.insert(
        Season::Summer,
        asset_server.load(config.clone().base_path + "Classroom/ClassroomSummer.png"),
    );
    res.seasons.insert(
        Season::Autumn,
        asset_server.load(config.clone().base_path + "Classroom/ClassroomAutumn.png"),
    );
    res.seasons.insert(
        Season::Winter,
        asset_server.load(config.clone().base_path + "Classroom/ClassroomWinter.png"),
    );

    let sprite = SpriteBundle {
        texture: spring2,
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
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
    res.entity = sprite_entity;
}

pub fn listen_events(
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    res: Res<RoomResources>,
    mut query: Query<&mut Handle<Image>>,
) {
    for e in season_changed_events.read() {
        if let Ok(mut texture_handle) = query.get_mut(res.entity) {
            if let Some(h) = res.seasons.get(&e.season) {
                *texture_handle = h.clone();
            }
        }
    }
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_events)
            .insert_resource(RoomResources::new())
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
pub struct RoomResources {
    entity: Entity,
    seasons: HashMap<Season, Handle<Image>>,
}

impl RoomResources {
    pub fn new() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            seasons: HashMap::new(),
        }
    }
}
