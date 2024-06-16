use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::config::Config;

const SCALE: f32 = 0.25;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(config.clone().base_path + "Cooking/cookingNoTeacher.png"),
        transform: Transform {
            //exactly the same coordinates in teacher display (overlaps)
            translation: Vec3 {
                x: 770.,
                y: 350.,
                z: 10.0,
            },
            scale: Vec3 {
                x: SCALE,
                y: SCALE,
                z: 1.,
            },
            ..default()
        },
        ..default()
    });

    let mut load = |id: FoodId, prefix: String, nb: i8| {
        let mut imgs: Vec<Handle<Image>> = Vec::new();
        //images name start at 1
        for i in 1..=nb {
            let path = format!("{}Cooking/{}{}.png", config.clone().base_path, prefix, i);
            imgs.push(asset_server.load(path));
        }
        data.food_textures.insert(id, imgs);
    };
    load(0, "FoodA".to_string(), 4);
    load(1, "FoodB".to_string(), 4);
    load(2, "FoodC".to_string(), 5);
    load(3, "FoodD".to_string(), 3);
    load(4, "FoodE".to_string(), 4);

    let mut register_food = |id: FoodId, pos: (f32, f32, f32)| {
        let texture = data.food_textures.get(&id).unwrap().get(0).unwrap().clone();
        let e = place_food(&mut commands, texture, pos);
        data.food_images.insert(id, e);
    };
    register_food(0, (625., 340., 20.));
    register_food(1, (680., 340., 21.));
    register_food(2, (770., 305., 22.));
    register_food(3, (850., 290., 23.));
    register_food(4, (905., 325., 20.));
}

fn place_food(commands: &mut Commands, texture: Handle<Image>, pos: (f32, f32, f32)) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3 {
                    x: pos.0,
                    y: pos.1,
                    z: pos.2,
                },
                scale: Vec3 {
                    x: SCALE,
                    y: SCALE,
                    z: 1.,
                },
                ..default()
            },
            // visibility: Visibility::Hidden,
            ..default()
        })
        .id()
}

pub struct KitchenViewPlugin;

impl Plugin for KitchenViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KitchenData::default())
            .add_systems(Startup, load_resources);
    }
}

type FoodId = i8;

#[derive(Resource, Default)]
struct KitchenData {
    food_images: HashMap<FoodId, Entity>,
    food_textures: HashMap<FoodId, Vec<Handle<Image>>>,
}
