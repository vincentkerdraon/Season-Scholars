use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::kitchen::*;

const SCALE: f32 = 0.25;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<KitchenData>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(
            config
                .base_path
                .join("images/ready/Cooking/cookingNoTeacher.png"),
        ),
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

    let load = |prefix: String, nb: i8| -> Vec<Handle<Image>> {
        let mut imgs: Vec<Handle<Image>> = Vec::new();
        //images name start at 1
        for i in 1..=nb {
            imgs.push(
                asset_server.load(
                    config
                        .base_path
                        .join(format!("images/ready/Cooking/{}{}.png", prefix, i)),
                ),
            );
        }
        imgs
    };
    let textures_0 = load("FoodA".to_string(), 4);
    let textures_1 = load("FoodB".to_string(), 4);
    let textures_2 = load("FoodC".to_string(), 5);
    let textures_3 = load("FoodD".to_string(), 3);
    let textures_4 = load("FoodE".to_string(), 4);

    let mut register_food = |id: FoodId, textures: Vec<Handle<Image>>, pos: (f32, f32, f32)| {
        let texture = textures.first().unwrap().clone();
        let e = place_food(&mut commands, texture, pos);
        let food_data = FoodData {
            e,
            displayed: true,
            texture_index: 0,
            textures,
            updated: true,
        };
        data.food.insert(id, food_data);
    };
    register_food(0, textures_0, (625., 340., 20.));
    register_food(1, textures_1, (680., 340., 21.));
    register_food(2, textures_2, (770., 305., 22.));
    register_food(3, textures_3, (850., 290., 23.));
    register_food(4, textures_4, (905., 325., 20.));

    data.dirty = true;
}

fn place_food(commands: &mut Commands, texture: Handle<Image>, pos: (f32, f32, f32)) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture,
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
            ..default()
        })
        .id()
}

fn draw(mut data: ResMut<KitchenData>, mut query: Query<(&mut Handle<Image>, &mut Visibility)>) {
    if !data.dirty {
        return;
    }
    data.dirty = false;

    for (_, food_data) in data.food.iter_mut() {
        if !food_data.updated {
            continue;
        }

        if let Ok((mut texture_handle, mut visibility)) = query.get_mut(food_data.e) {
            if !food_data.displayed {
                *visibility = Visibility::Hidden;
                continue;
            }
            *visibility = Visibility::Visible;
            *texture_handle = food_data
                .textures
                .get(food_data.texture_index)
                .unwrap()
                .clone();
        }
    }
}

fn listen_events(
    mut cook_events: EventReader<CookedEvent>,
    mut students_eat_events: EventReader<StudentsEatEvent>,
    mut teacher_eat_events: EventReader<TeacherAteEvent>,
    mut data: ResMut<KitchenData>,
) {
    let mut food_remaining: Option<i8> = None;

    for e in cook_events.read() {
        food_remaining = Some(e.food_remaining);
    }
    for e in students_eat_events.read() {
        food_remaining = Some(e.food_remaining);
    }
    for e in teacher_eat_events.read() {
        food_remaining = Some(e.food_remaining);
    }

    if let Some(food_remaining) = food_remaining {
        data.update_food_remaining(food_remaining);
    }
}

pub struct KitchenViewPlugin;

impl Plugin for KitchenViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KitchenData::default())
            .add_systems(PreUpdate, listen_events)
            .add_systems(Update, draw)
            .add_systems(Startup, load_resources);
    }
}

type FoodId = i8;
struct FoodData {
    e: Entity,
    displayed: bool,
    texture_index: usize,
    updated: bool,
    textures: Vec<Handle<Image>>,
}

#[derive(Resource, Default)]
struct KitchenData {
    /// position_on_table => (image, displayed, texture_index, updated, textures)
    food: HashMap<FoodId, FoodData>,
    dirty: bool,
}

impl KitchenData {
    fn update_food_remaining(&mut self, food_remaining: FoodRemaining) {
        let current = self
            .food
            .iter()
            .filter(|(_, food_data)| food_data.displayed)
            .count() as FoodRemaining;
        if current == food_remaining {
            return;
        }
        self.dirty = true;

        if current > food_remaining {
            let mut remove_nb = current - food_remaining;

            for (_, food_data) in self
                .food
                .iter_mut()
                .filter(|(_, food_data)| food_data.displayed)
            {
                food_data.displayed = false;
                food_data.updated = true;
                remove_nb -= 1;
                if remove_nb <= 0 {
                    return;
                }
            }
        }

        let mut add_nb = food_remaining - current;

        for (_, food_data) in self
            .food
            .iter_mut()
            .filter(|(_, food_data)| !food_data.displayed)
        {
            food_data.displayed = true;
            food_data.updated = true;

            let food_id_max = food_data.textures.len();
            food_data.texture_index += 1;
            if food_data.texture_index >= food_id_max {
                food_data.texture_index = 0;
            }

            add_nb -= 1;
            if add_nb <= 0 {
                return;
            }
        }
    }
}
