use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::controllers::portal::events::*, config::Config, model::definitions::Season,
};

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<PortalResources>,
) {
    data.window_closed = asset_server.load(config.clone().base_path + "Windows/Window1Closed.png");
    data.window_available =
        asset_server.load(config.clone().base_path + "Windows/Window1Opened.png");
    data.seasons.insert(
        Season::Spring,
        asset_server.load(config.clone().base_path + "Windows/Window1HarvestA.png"),
    );
    data.seasons.insert(
        Season::Summer,
        asset_server.load(config.clone().base_path + "Windows/Window1HarvestC.png"),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(config.clone().base_path + "Windows/Window1HarvestM.png"),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(config.clone().base_path + "Windows/Window1HarvestL.png"),
    );
    //images name start at 1
    for i in 1..=config.clone().portal_opened_nb {
        let path = format!("{}Door/DoorMonster{}.png", config.clone().base_path, i);
        data.portal_opened.push(asset_server.load(path));
    }
    //images name start at 1
    for i in 1..=config.clone().portal_closed_nb {
        let path = format!("{}Door/DoorProtected{}.png", config.clone().base_path, i);
        data.portal_closed.push(asset_server.load(path));
    }
    data.portal = commands
        .spawn(SpriteBundle {
            texture: data.get_closed_auto().unwrap().clone(),
            transform: Transform {
                translation: Vec3 {
                    x: -340.0,
                    y: 310.0,
                    z: 50.0,
                },
                scale: Vec3 {
                    x: 0.40,
                    y: 0.40,
                    z: 1.,
                },
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .id();

    let e = place_need(&mut commands, &mut data, -545., 290., 0.55);
    data.needs.insert((0, 0), e);
    let e = place_need(&mut commands, &mut data, -545., 190., 0.55);
    data.needs.insert((0, 1), e);
    let e = place_need(&mut commands, &mut data, -545., 90., 0.55);
    data.needs.insert((0, 2), e);
    let e = place_window(&mut commands, &mut data, -545., 190., 0.55, 0.53);
    data.windows.insert(0, e);

    let e = place_need(&mut commands, &mut data, -620., 260., 0.53);
    data.needs.insert((1, 0), e);
    let e = place_need(&mut commands, &mut data, -620., 160., 0.53);
    data.needs.insert((1, 1), e);
    let e = place_need(&mut commands, &mut data, -620., 60., 0.53);
    data.needs.insert((1, 2), e);
    let e = place_window(&mut commands, &mut data, -620., 158., 0.54, 0.57);
    data.windows.insert(1, e);

    let e = place_need(&mut commands, &mut data, -710., 230., 0.62);
    data.needs.insert((2, 0), e);
    let e = place_need(&mut commands, &mut data, -710., 120., 0.62);
    data.needs.insert((2, 1), e);
    let e = place_need(&mut commands, &mut data, -710., 10., 0.62);
    data.needs.insert((2, 2), e);
    let e = place_window(&mut commands, &mut data, -709., 120., 0.63, 0.64);
    data.windows.insert(2, e);

    let e = place_need(&mut commands, &mut data, -790., 200., 0.69);
    data.needs.insert((3, 0), e);
    let e = place_need(&mut commands, &mut data, -790., 70., 0.69);
    data.needs.insert((3, 1), e);
    let e = place_need(&mut commands, &mut data, -790., -60., 0.69);
    data.needs.insert((3, 2), e);
    let e = place_window(&mut commands, &mut data, -800., 70., 0.69, 0.69);
    data.windows.insert(3, e);
}

pub fn place_window(
    commands: &mut Commands,
    data: &mut PortalResources,
    pos_x: f32,
    pos_: f32,
    scale_x: f32,
    scale_y: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: data.window_closed.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: pos_x,
                    y: pos_,
                    z: 49.0,
                },
                scale: Vec3 {
                    x: scale_x,
                    y: scale_y,
                    z: 1.,
                },
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .id()
}

pub fn place_need(
    commands: &mut Commands,
    data: &mut PortalResources,
    x: f32,
    y: f32,
    scale: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: data.seasons.get(&Season::Autumn).unwrap().clone(),
            transform: Transform {
                translation: Vec3 { x, y, z: 50.0 },
                scale: Vec3 {
                    x: scale,
                    y: scale,
                    z: 1.,
                },
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .id()
}
pub fn listen_events(
    mut portal_observed_events: EventReader<PortalObservedEvent>,
    mut monster_popped_events: EventReader<MonsterPoppedEvent>,
    mut portal_attacked_events: EventReader<PortalAttackedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut data: ResMut<PortalResources>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
) {
    let mut dirty = false;
    let mut monsters: Vec<Monster> = Vec::new();
    let mut health: i8 = 0;

    if let Some(e) = monster_popped_events.read().last() {
        monsters = e.monsters.clone();
        dirty = true;
    }

    if let Some(e) = portal_attacked_events.read().last() {
        health = e.health;
        monsters = e.monsters.clone();
        dirty = true;
    }

    if let Some(e) = monster_fed_events.read().last() {
        monsters = e.monsters.clone();
        dirty = true;
    }

    if let Some(e) = portal_observed_events.read().last() {
        health = e.health;
        monsters = e.monsters.clone();
        dirty = true;
    }

    if !dirty {
        return;
    }

    for i in 0..=3 {
        let monster_new = monsters.get(i);
        let monster_old = data.monsters.get(i);

        if should_redraw_monster(monster_new, monster_old) {
            let mut needs = Vec::new();
            let mut revealed = false;
            if let Some(new) = monster_new {
                needs = new.needs.clone();
                revealed = new.revealed;
            }
            display_window(&mut data, &mut query, i as i8, revealed, needs);

            if i == 0 {
                display_monster(&mut data, &mut query, revealed);
            }
        }
    }

    data.monsters = monsters;
    data.health = health;
}

fn display_window(
    data: &mut PortalResources,
    query: &mut Query<(&mut Handle<Image>, &mut Visibility)>,
    i: i8,
    revealed: bool,
    needs: Vec<Season>,
) {
    //!revealed && need.len == 0 => closed
    //!revealed && need.len > 0 => opened
    //revealed && need.len > 0 => needs

    if let Ok((mut texture_handle, mut visibility)) = query.get_mut(*data.windows.get(&i).unwrap())
    {
        if !revealed && needs.len() == 0 {
            *visibility = Visibility::Visible;
            *texture_handle = data.window_closed.clone();
        } else if !revealed && needs.len() > 0 {
            *visibility = Visibility::Visible;
            *texture_handle = data.window_available.clone();
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    for j in 0..=2 {
        if let Ok((mut texture_handle, mut visibility)) =
            query.get_mut(*data.needs.get(&(i, j)).unwrap())
        {
            *visibility = Visibility::Hidden;
            if revealed {
                if let Some(n) = needs.get(j as usize) {
                    *visibility = Visibility::Visible;
                    *texture_handle = data.seasons.get(n).unwrap().clone();
                }
            }
        }
    }
}

fn display_monster(
    data: &mut PortalResources,
    query: &mut Query<(&mut Handle<Image>, &mut Visibility)>,
    revealed: bool,
) {
    if let Ok((mut texture_handle, mut visibility)) = query.get_mut(data.portal) {
        *visibility = Visibility::Visible;
        if !revealed {
            *texture_handle = data.get_closed_auto().unwrap().clone();
        } else {
            *texture_handle = data.get_opened_auto().unwrap().clone();
        }
    }
}

fn should_redraw_monster(monster_new: Option<&Monster>, monster_old: Option<&Monster>) -> bool {
    // with new, without old => redraw all
    // without new, with old => redraw all
    // with new, with old, different id => redraw all
    // with new, with old, same id, different needs => redraw all
    // with new, with old, same id, same needs => no change
    // without new, without old => no change

    if monster_new.is_none() && monster_old.is_none() {
        return false;
    }
    if monster_new.is_none() && monster_old.is_some() {
        return true;
    }
    if monster_new.is_some() && monster_old.is_none() {
        return true;
    }
    let new = monster_new.unwrap();
    let old = monster_old.unwrap();

    if new.id != old.id {
        return true;
    }
    if new.needs != old.needs {
        return true;
    }

    false
}

pub struct PortalViewPlugin;

impl Plugin for PortalViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_events)
            .insert_resource(PortalResources::new())
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
pub struct PortalResources {
    // 0 => current monster ; 1 => next monster
    windows: HashMap<i8, Entity>,
    /// (x=0,y=2) => third need for current monster ; (x=1,y=0) => first need for next waiting monster
    needs: HashMap<(i8, i8), Entity>,
    seasons: HashMap<Season, Handle<Image>>,
    window_closed: Handle<Image>,
    window_available: Handle<Image>,

    portal: Entity,
    portal_closed: Vec<Handle<Image>>,
    portal_closed_last_used_index: usize,
    portal_opened: Vec<Handle<Image>>,
    portal_opened_last_used_index: usize,

    monsters: Vec<Monster>,
    health: i8,
}

impl PortalResources {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            window_closed: Handle::default(),
            window_available: Handle::default(),
            needs: HashMap::new(),
            seasons: HashMap::new(),
            portal: Entity::PLACEHOLDER,
            portal_opened: Vec::new(),
            portal_closed: Vec::new(),
            portal_opened_last_used_index: 0,
            portal_closed_last_used_index: 0,
            health: 0,
            monsters: Vec::new(),
        }
    }

    pub fn get_opened_auto(&mut self) -> Option<&Handle<Image>> {
        self.portal_opened_last_used_index = self.portal_opened_last_used_index + 1;
        if self.portal_opened_last_used_index == self.portal_opened.len() {
            self.portal_opened_last_used_index = 0;
        }
        self.portal_opened.get(self.portal_opened_last_used_index)
    }

    pub fn get_closed_auto(&mut self) -> Option<&Handle<Image>> {
        self.portal_closed_last_used_index = self.portal_closed_last_used_index + 1;
        if self.portal_closed_last_used_index == self.portal_closed.len() {
            self.portal_closed_last_used_index = 0;
        }
        self.portal_closed.get(self.portal_closed_last_used_index)
    }
}
