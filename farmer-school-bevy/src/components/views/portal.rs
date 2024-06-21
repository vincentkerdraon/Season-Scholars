use std::collections::HashMap;
use std::num::Wrapping;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::overlord::{GameOverEvent, ResetGameEvent};
use crate::model::portal::*;
use crate::model::season::Season;

const PORTAL_OPENED_NB: i8 = 5;
const PORTAL_CLOSED_NB: i8 = 10;
// 5 steps and attack on next step
const ATTACK_PROGRESS_INCREMENTS: i8 = 6;

fn load_attack_progress(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
) {
    let texture0 = asset_server.load(config.base_path.join("images/ready/Door/ProgressBar0.png"));
    let texture1 = asset_server.load(config.base_path.join("images/ready/Door/ProgressBar1.png"));
    let texture2 = asset_server.load(config.base_path.join("images/ready/Door/ProgressBar2.png"));
    let texture3 = asset_server.load(config.base_path.join("images/ready/Door/ProgressBar3.png"));
    let texture4 = asset_server.load(config.base_path.join("images/ready/Door/ProgressBar4.png"));

    data.attack_progress = vec![];

    let mut register_attack = |texture: Handle<Image>, pos: (f32, f32), scale: (f32, f32)| {
        let e = commands
            .spawn(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3 {
                        x: pos.0,
                        y: pos.1,
                        z: 58.0,
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
            .id();
        data.attack_progress.push(e);
    };

    register_attack(texture0.clone(), (-560., 480.), (0.2, 0.2));
    register_attack(texture1.clone(), (-530., 440.), (0.2, 0.2));
    register_attack(texture2.clone(), (-570., 380.), (0.2, 0.2));
    register_attack(texture3.clone(), (-565., 330.), (0.2, 0.2));
    register_attack(texture4.clone(), (-540., 210.), (0.2, 0.2));
}

fn load_windows(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
) {
    data.window_closed = asset_server.load(
        config
            .base_path
            .join("images/ready/Windows/Window1Closed.png"),
    );
    data.window_available = asset_server.load(
        config
            .base_path
            .join("images/ready/Windows/Window1Opened.png"),
    );
    data.seasons.insert(
        Season::Spring,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Windows/Window1HarvestA.png"),
        ),
    );
    data.seasons.insert(
        Season::Summer,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Windows/Window1HarvestC.png"),
        ),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Windows/Window1HarvestM.png"),
        ),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(
            config
                .base_path
                .join("images/ready/Windows/Window1HarvestL.png"),
        ),
    );

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

fn load_portal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
) {
    //images name start at 1
    for i in 1..=PORTAL_OPENED_NB {
        data.portal_opened.push(
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Door/DoorMonster{}.png", i)),
            ),
        );
    }
    //images name start at 1
    for i in 1..=PORTAL_CLOSED_NB {
        data.portal_closed.push(
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Door/DoorProtected{}.png", i)),
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
    data: &PortalData,
    query: &mut Query<(&mut Handle<Image>, &mut Visibility)>,
    i: i8,
    window_revealed: bool,
    needs: &[Season],
) {
    //!revealed && need.len == 0 => closed
    //!revealed && need.len > 0 => opened
    //revealed && need.len > 0 => needs

    if let Ok((mut texture_handle, mut visibility)) = query.get_mut(*data.windows.get(&i).unwrap())
    {
        if !window_revealed && needs.is_empty() {
            *visibility = Visibility::Visible;
            *texture_handle = data.window_closed.clone();
        } else if !window_revealed && !needs.is_empty() {
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
            if window_revealed {
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
    monster_visible: bool,
) {
    if let Ok((mut texture_handle, mut visibility)) = query.get_mut(data.portal) {
        *visibility = Visibility::Visible;
        if !monster_visible {
            *texture_handle = data.get_closed_auto().unwrap().clone();
        } else {
            *texture_handle = data.get_opened_auto().unwrap().clone();
        }
    }
}

fn display_health(
    data: &mut PortalData,
    query: &mut Query<(&mut Handle<Image>, &mut Visibility)>,
    health: &PortalHealth,
) {
    let mut health = *health;
    for e in data.attack_progress.iter() {
        if let Ok((_, mut visibility)) = query.get_mut(*e) {
            health -= 1;
            if health > 0 {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn should_redraw_window(monster_new: &Option<&Monster>, monster_old: &Option<&Monster>) -> bool {
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

    if new.window_revealed != old.window_revealed {
        return true;
    }
    if new.needs != old.needs {
        return true;
    }

    false
}

fn should_redraw_monster(monster_new: &Option<&Monster>, monster_old: &Option<&Monster>) -> bool {
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
    if new.monster_visible != old.monster_visible {
        return true;
    }

    false
}

fn display_attack_progress(
    time: Res<Time>,
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
) {
    if !data.activated {
        return;
    }

    data.frame += Wrapping(1);
    if !data.frame.0 % config.draw_frame_modulo != 0 {
        return;
    }

    let now = time.elapsed_seconds_f64();

    let mut progress = ATTACK_PROGRESS_INCREMENTS;
    if let Some(monster) = data.monsters.first() {
        if data.attack_progress_next_s != monster.next_attack_s {
            data.attack_progress_next_s = monster.next_attack_s;
            data.attack_progress_reset_s = now;
        }
        let total_time_s = data.attack_progress_next_s - data.attack_progress_reset_s;
        let remaining_time_s = now - data.attack_progress_reset_s;
        progress = (remaining_time_s / total_time_s * (ATTACK_PROGRESS_INCREMENTS as f64)) as i8;
    }

    for i in 0..ATTACK_PROGRESS_INCREMENTS {
        if let Some(e) = data.attack_progress.get(i as usize) {
            if let Ok((_, mut visibility)) = query.get_mut(*e) {
                if i < progress {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn listen_events(
    config: Res<Config>,
    mut data: ResMut<PortalData>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
    mut portal_observed_events: EventReader<PortalObservedEvent>,
    mut monster_popped_events: EventReader<MonsterPoppedEvent>,
    mut portal_attacked_events: EventReader<PortalAttackedEvent>,
    mut monster_fed_events: EventReader<MonsterFedEvent>,
    mut portal_fixed_events: EventReader<PortalFixedEvent>,
) {
    let mut dirty = false;
    let mut monsters: Vec<Monster> = Vec::new();
    let mut health: Option<PortalHealth> = None;

    if let Some(e) = portal_attacked_events.read().last() {
        health = Some(e.health);
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = portal_fixed_events.read().last() {
        health = Some(e.health);
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = monster_fed_events.read().last() {
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    //monster_popped_events must be after monster_fed_events
    if let Some(e) = monster_popped_events.read().last() {
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if let Some(e) = portal_observed_events.read().last() {
        health = Some(e.health);
        monsters.clone_from(&e.monsters);
        dirty = true;
    }

    if !dirty {
        return;
    }

    if let Some(health) = health {
        if health != data.health {
            data.health = health;
            display_health(&mut data, &mut query, &health);
        }
    }

    let monster_new = &monsters.first();
    let monster_old = &data.monsters.first().clone();
    if should_redraw_monster(monster_new, monster_old) {
        if let Some(new) = monster_new {
            display_monster(&mut data, &mut query, new.monster_visible);
        } else {
            display_monster(&mut data, &mut query, false);
        }
    }

    for i in 0..config.portal_windows_nb {
        let monster_new = &monsters.get(i as usize);
        let monster_old = &data.monsters.get(i as usize);

        if should_redraw_window(monster_new, monster_old) {
            if let Some(new) = monster_new {
                display_window(
                    &config,
                    &data,
                    &mut query,
                    i,
                    new.window_revealed,
                    &new.needs,
                );
            } else {
                display_window(&config, &data, &mut query, i, false, &Vec::new());
            }
        }
    }

    data.monsters = monsters;
}

fn listen_game_over(
    mut data: ResMut<PortalData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.activated = false;
}

fn listen_reset(mut data: ResMut<PortalData>, mut reset_game_events: EventReader<ResetGameEvent>) {
    if reset_game_events.read().last().is_some() {
        data.activated = true;
    }
}

pub struct PortalViewPlugin;

impl Plugin for PortalViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PortalData::new())
            .add_systems(Startup, load_portal)
            .add_systems(Startup, load_windows)
            .add_systems(Startup, load_attack_progress)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(Update, display_attack_progress)
            .add_systems(Update, listen_events);
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

    attack_progress: Vec<Entity>,
    attack_progress_next_s: f64,
    attack_progress_reset_s: f64,

    monsters: Vec<Monster>,
    health: i8,

    activated: bool,
    frame: Wrapping<i8>,
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
            attack_progress: Vec::new(),
            attack_progress_next_s: 0.,
            attack_progress_reset_s: 0.,
            activated: false,
            frame: Wrapping(0),
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
