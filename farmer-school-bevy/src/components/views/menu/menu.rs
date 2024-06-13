use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::controllers::{
        overlord::events::{
            DisplayScreenGameEvent, DisplayScreenGameOverRecapEvent, DisplayScreenMenuEvent,
        },
        season::events::SeasonChangedEvent,
    },
    config::Config,
    model::definitions::{Season, Teacher},
};

fn display(mut commands: Commands, mut data: ResMut<MenuResources>) {
    if !data.dirty {
        return;
    }
    data.dirty = false;

    if !data.display {
        //FIXME hide everything
        return;
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut data: ResMut<MenuResources>,
) {
    data.player_a = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "PlayerA ready",
                TextStyle {
                    font_size: 60.0,
                    color: Color::rgb(0., 255., 0.),
                    ..default()
                },
            ),
            ..default()
        })
        .id();

    // let spring = asset_server.load(config.clone().base_path + "Classroom/ClassroomSpring.png");
    // let spring2 = spring.clone();

    // res.seasons.insert(Season::Autumn, spring);
    // res.seasons.insert(
    //     Season::Summer,
    //     asset_server.load(config.clone().base_path + "Classroom/ClassroomSummer.png"),
    // );
    // res.seasons.insert(
    //     Season::Autumn,
    //     asset_server.load(config.clone().base_path + "Classroom/ClassroomAutumn.png"),
    // );
    // res.seasons.insert(
    //     Season::Winter,
    //     asset_server.load(config.clone().base_path + "Classroom/ClassroomWinter.png"),
    // );

    // let sprite = SpriteBundle {
    //     texture: spring2,
    //     transform: Transform {
    //         translation: Vec3 {
    //             x: 0.,
    //             y: 0.,
    //             z: -1.,
    //         },
    //         scale: Vec3 {
    //             x: 0.73,
    //             y: 0.73,
    //             z: 1.,
    //         },
    //         ..default()
    //     },
    //     ..default()
    // };
    // let sprite_entity = commands.spawn(sprite).id();
    // res.entity = sprite_entity;
}

pub fn listen_events(
    mut data: ResMut<MenuResources>,
    mut display_screen_menu_events: EventReader<DisplayScreenMenuEvent>,
    mut display_screen_game_events: EventReader<DisplayScreenGameEvent>,
    mut display_screen_game_over_recap_events: EventReader<DisplayScreenGameOverRecapEvent>,
) {
    for _ in display_screen_game_events.read() {
        data.display = false;
        data.dirty = true;
    }
    for _ in display_screen_game_over_recap_events.read() {
        data.display = false;
        data.dirty = true;
    }
    for e in display_screen_menu_events.read() {
        data.display = true;
        data.teachers = e.teachers.clone();
        data.dirty = true;
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuResources::new())
            .add_systems(PreUpdate, listen_events)
            .add_systems(Update, display)
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
pub struct MenuResources {
    player_a: Entity,

    dirty: bool,
    display: bool,
    teachers: Vec<Teacher>,
    // entity: Entity,
}

impl MenuResources {
    pub fn new() -> Self {
        Self {
            player_a: Entity::PLACEHOLDER,
            dirty: false,
            display: false,
            teachers: Vec::new(),
        }
    }
}
