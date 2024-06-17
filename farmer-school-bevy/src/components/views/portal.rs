use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::portal::*;
use crate::model::season::Season;

const PORTAL_OPENED_NB: i8 = 5;
const PORTAL_CLOSED_NB: i8 = 10;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
) {
    data.window_closed = asset_server.load(config.base_path.join("Windows/Window1Closed.png"));
    data.window_available = asset_server.load(config.base_path.join("Windows/Window1Opened.png"));
    data.seasons.insert(
        Season::Spring,
        asset_server.load(config.base_path.join("Windows/Window1HarvestA.png")),
    );
    data.seasons.insert(
        Season::Summer,
        asset_server.load(config.base_path.join("Windows/Window1HarvestC.png")),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(config.base_path.join("Windows/Window1HarvestM.png")),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(config.base_path.join("Windows/Window1HarvestL.png")),
    );
    //images name start at 1
    for i in 1..=PORTAL_OPENED_NB {
        data.portal_opened
            .push(asset_server.load(config.base_path.join(format!("Door/DoorMonster{}.png", i))));
    }
    //images name start at 1
    for i in 1..=PORTAL_CLOSED_NB {
        data.portal_closed.push(
            asset_server.load(
                config
                    .base_path
                    .join(format!("Door/DoorProtected{}.png", i)),
            ),
        );
    }
    data.portal = commands
        .spawn(SpriteBundle {
            texture: data.get_closed_auto().unwrap().clone(),
            transform: Transform {
                translation: Vec3 {
                    x: -340.0,
                    y: 310.0,
                    z: 5.0,
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

    let e = place_need(&mut commands, &mut data, (-545., 290.), 0.55);
    data.needs.insert((0, 0), e);
    let e = place_need(&mut commands, &mut data, (-545., 190.), 0.55);
    data.needs.insert((0, 1), e);
    let e = place_need(&mut commands, &mut data, (-545., 90.), 0.55);
    data.needs.insert((0, 2), e);
    let e = place_window(&mut commands, &mut data, (-545., 190.), (0.55, 0.53));
    data.windows.insert(0, e);

    let e = place_need(&mut commands, &mut data, (-620., 260.), 0.53);
    data.needs.insert((1, 0), e);
    let e = place_need(&mut commands, &mut data, (-620., 160.), 0.53);
    data.needs.insert((1, 1), e);
    let e = place_need(&mut commands, &mut data, (-620., 60.), 0.53);
    data.needs.insert((1, 2), e);
    let e = place_window(&mut commands, &mut data, (-620., 158.), (0.54, 0.57));
    data.windows.insert(1, e);

    let e = place_need(&mut commands, &mut data, (-710., 230.), 0.62);
    data.needs.insert((2, 0), e);
    let e = place_need(&mut commands, &mut data, (-710., 120.), 0.62);
    data.needs.insert((2, 1), e);
    let e = place_need(&mut commands, &mut data, (-710., 10.), 0.62);
    data.needs.insert((2, 2), e);
    let e = place_window(&mut commands, &mut data, (-709., 120.), (0.63, 0.64));
    data.windows.insert(2, e);

    let e = place_need(&mut commands, &mut data, (-790., 200.), 0.69);
    data.needs.insert((3, 0), e);
    let e = place_need(&mut commands, &mut data, (-790., 70.), 0.69);
    data.needs.insert((3, 1), e);
    let e = place_need(&mut commands, &mut data, (-790., -60.), 0.69);
    data.needs.insert((3, 2), e);
    let e = place_window(&mut commands, &mut data, (-800., 70.), (0.69, 0.69));
    data.windows.insert(3, e);
}

fn place_window(
    commands: &mut Commands,
    data: &mut PortalData,
    pos: (f32, f32),
    scale: (f32, f32),
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: data.window_closed.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: pos.0,
                    y: pos.1,
                    z: 49.0,
                },
                scale: Vec3 {
                    x: scale.0,
                    y: scale.1,
                    z: 1.,
                },
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .id()
}

fn place_need(
    commands: &mut Commands,
    data: &mut PortalData,
    pos: (f32, f32),
    scale: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: data.seasons.get(&Season::Autumn).unwrap().clone(),
            transform: Transform {
                translation: Vec3 {
                    x: pos.0,
                    y: pos.1,
                    z: 50.0,
                },
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
fn display_window(
    config: &Config,
    data: &mut PortalData,
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
        if !revealed && needs.is_empty() {
            *visibility = Visibility::Visible;
            *texture_handle = data.window_closed.clone();
        } else if !revealed && !needs.is_empty() {
            *visibility = Visibility::Visible;
            *texture_handle = data.window_available.clone();
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    for j in 0..config.portal_windows_seasons_nb {
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
    data: &mut PortalData,
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
    if new.revealed != old.revealed {
        return true;
    }
    if new.needs != old.needs {
        return true;
    }

    false
}

fn listen_events(
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
    mut portal_observed_events: EventReader<PortalObservedEvent>,
    mut monster_popped_events: EventReader<MonsterPoppedEvent>,
    mut portal_attacked_events: EventReader<PortalAttackedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
) {
    let mut dirty = false;
    let mut monsters: Vec<Monster> = Vec::new();
    let mut health: i8 = 0;

    if let Some(e) = monster_popped_events.read().last() {
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = portal_attacked_events.read().last() {
        health = e.health;
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = monster_fed_events.read().last() {
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = portal_observed_events.read().last() {
        health = e.health;
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if !dirty {
        return;
    }

    for i in 0..config.portal_windows_nb {
        let monster_new = monsters.get(i as usize);
        let monster_old = data.monsters.get(i as usize);

        if should_redraw_monster(monster_new, monster_old) {
            let mut needs = Vec::new();
            let mut revealed = false;
            if let Some(new) = monster_new {
                needs.clone_from(&new.needs);
                revealed = new.revealed;
            }
            display_window(&config, &mut data, &mut query, i, revealed, needs);

            if i == 0 {
                display_monster(&mut data, &mut query, revealed);
            }
        }
    }

    data.monsters = monsters;
    data.health = health;
}

pub struct PortalViewPlugin;

impl Plugin for PortalViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_events)
            .insert_resource(PortalData::new())
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
struct PortalData {
    /// 0 => current monster ; 1 => next monster
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

impl PortalData {
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
        self.portal_opened_last_used_index += 1;
        if self.portal_opened_last_used_index == self.portal_opened.len() {
            self.portal_opened_last_used_index = 0;
        }
        self.portal_opened.get(self.portal_opened_last_used_index)
    }

    pub fn get_closed_auto(&mut self) -> Option<&Handle<Image>> {
        self.portal_closed_last_used_index += 1;
        if self.portal_closed_last_used_index == self.portal_closed.len() {
            self.portal_closed_last_used_index = 0;
        }
        self.portal_closed.get(self.portal_closed_last_used_index)
    }
}
