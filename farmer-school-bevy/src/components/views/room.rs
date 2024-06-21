use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::portal::{
    MonsterPoppedEvent, PortalAttackedEvent, PortalFixedEvent, PortalHealth, PortalObservedEvent,
};
use crate::model::season::*;

fn load_resources_top(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<RoomData>,
) {
    let spring = asset_server.load(
        config
            .base_path
            .join("images/ready/Classroom/classroom_spring_no_floor.png"),
    );

    data.seasons.insert(Season::Spring, spring.clone());
    data.seasons.insert(
        Season::Summer,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_summer_no_floor.png"),
        ),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_autumn_no_floor.png"),
        ),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_winter_no_floor.png"),
        ),
    );

    let sprite = SpriteBundle {
        texture: spring.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: -30.,
                z: 2.,
            },
            scale: Vec3 {
                x: 0.73,
                y: 0.73,
                z: 1.,
            },
            ..default()
        },
        visibility: Visibility::Visible,
        ..default()
    };
    data.top = commands.spawn(sprite).id();
}

fn load_resources_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<RoomData>,
) {
    data.floors.insert(
        1,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_floor_health_1.png"),
        ),
    );
    data.floors.insert(
        2,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_floor_health_2.png"),
        ),
    );
    data.floors.insert(
        3,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Classroom/classroom_floor_health_3.png"),
        ),
    );
    let start = asset_server.load(
        config
            .base_path
            .join("images/ready/Classroom/classroom_floor_health_4.png"),
    );
    data.floors.insert(4, start.clone());

    let sprite = SpriteBundle {
        texture: start.clone(),
        transform: Transform {
            translation: Vec3 {
                x: -20.,
                y: -30.,
                z: 1.,
            },
            scale: Vec3 {
                x: 0.80,
                y: 0.80,
                z: 1.,
            },
            ..default()
        },
        visibility: Visibility::Visible,
        ..default()
    };
    data.floor = commands.spawn(sprite).id();
}

fn listen_events_season(
    mut season_changed_events: EventReader<SeasonChangedEvent>,
    data: Res<RoomData>,
    mut query: Query<&mut Handle<Image>>,
) {
    for e in season_changed_events.read() {
        if let Ok(mut texture_handle) = query.get_mut(data.top) {
            if let Some(h) = data.seasons.get(&e.season) {
                *texture_handle = h.clone();
            }
        }
    }
}

fn listen_events_floor(
    mut portal_observed_events: EventReader<PortalObservedEvent>,
    mut portal_fixed_events: EventReader<PortalFixedEvent>,
    mut portal_attacked_events: EventReader<PortalAttackedEvent>,
    mut monster_popped_events: EventReader<MonsterPoppedEvent>,
    data: Res<RoomData>,
    mut query: Query<&mut Handle<Image>>,
    config: Res<Config>,
) {
    let mut health: Option<PortalHealth> = None;

    if let Some(e) = monster_popped_events.read().last() {
        health = Some(e.health);
    }
    if let Some(e) = portal_attacked_events.read().last() {
        health = Some(e.health);
    }
    if let Some(e) = portal_fixed_events.read().last() {
        health = Some(e.health);
    }
    if let Some(e) = portal_observed_events.read().last() {
        health = Some(e.health);
    }

    if let Some(mut health) = health {
        if health < 1 {
            //keep displaying something after game over
            health = 1;
        }
        if health >= config.portal_health_max {
            //this is for debug, when the portal has more health than default
            //still display the max health image
            health = config.portal_health_max;
        }

        if let Ok(mut texture_handle) = query.get_mut(data.floor) {
            if let Some(h) = data.floors.get(&health) {
                *texture_handle = h.clone();
            }
        }
    }
}

pub struct RoomViewPlugin;

impl Plugin for RoomViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoomData::new())
            .add_systems(Startup, load_resources_top)
            .add_systems(Startup, load_resources_floor)
            .add_systems(Update, listen_events_floor)
            .add_systems(Update, listen_events_season);
    }
}

#[derive(Resource)]
struct RoomData {
    top: Entity,
    floor: Entity,
    seasons: HashMap<Season, Handle<Image>>,
    floors: HashMap<PortalHealth, Handle<Image>>,
}

impl RoomData {
    pub fn new() -> Self {
        Self {
            top: Entity::PLACEHOLDER,
            floor: Entity::PLACEHOLDER,
            seasons: HashMap::new(),
            floors: HashMap::new(),
        }
    }
}
