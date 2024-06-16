use std::{collections::HashMap, num::Wrapping};

use bevy::prelude::*;

use crate::{
    components::{
        controllers::{
            overlord::events::{GameOverEvent, InvalidActionStationEvent, ResetGameEvent},
            player_input::events::PlayerInputEvent,
            portal::events::PortalObservedEvent,
            students::events::TaughtEvent,
            teacher::events::TeacherMovedEvent,
            welcome::events::StudentWelcomedEvent,
        },
        moves::moves::possible_move,
    },
    model::{
        config::Config,
        definitions::{Reaction, Station, Teacher},
    },
};

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<TeacherData>,
) {
    let reaction_fail = asset_server.load(config.clone().base_path + "Reactions/fail.png");
    let reaction_success_long =
        asset_server.load(config.clone().base_path + "Reactions/successLong.png");
    let reaction_success_short =
        asset_server.load(config.clone().base_path + "Reactions/successShort.png");

    let teacher_a_teaching = asset_server
        .load(config.clone().base_path + "Teacher/TeacherA/TeacherATeachingWelcoming.png");
    let teacher_a_protecting =
        asset_server.load(config.clone().base_path + "Teacher/TeacherA/teacherAProtecting.png");
    let teacher_a_cooking =
        asset_server.load(config.clone().base_path + "Cooking/cookingWithTeacher.png");
    //FIXME
    // let teacher_a_watch =
    //     asset_server.load(config.clone().base_path + "Teacher/TeacherA/teacherAWatch.png");

    let path_left_center =
        asset_server.load(config.clone().base_path + "Path/ST_COL_LEFT_TO_ST_COL_CENTER.png");
    let path_center_right =
        asset_server.load(config.clone().base_path + "Path/ST_COL_RIGHT_TO_ST_COL_CENTER.png");
    let path_right_kitchen =
        asset_server.load(config.clone().base_path + "Path/ST_COL_RIGHT_TO_COOKING.png");
    let path_kitchen_welcome =
        asset_server.load(config.clone().base_path + "Path/WELCOME_TO_COOKING.png");
    let path_welcome_portal =
        asset_server.load(config.clone().base_path + "Path/WELCOME_TO_DOOR.png");
    let path_portal_left =
        asset_server.load(config.clone().base_path + "Path/ST_COL_LEFT_TO_WINDOWS.png");

    let e = place_reaction(&mut commands, reaction_fail, 0., 0., 1.);
    data.reactions.insert((Station::Welcome, Reaction::Fail), e);
    let e = place_reaction(&mut commands, reaction_success_long, 0., 0., 1.);
    data.reactions.insert((Station::Welcome, Reaction::Long), e);
    let e = place_reaction(&mut commands, reaction_success_short, 0., 0., 1.);
    data.reactions
        .insert((Station::Welcome, Reaction::Short), e);

    let e = place_teacher(
        &mut commands,
        teacher_a_teaching.clone(),
        (450., 380., 10.),
        (-0.15, 0.15),
    );
    data.teachers.insert(Station::Welcome, e);
    let e = place_teacher(
        &mut commands,
        teacher_a_teaching.clone(),
        (630., -230., 50.),
        (0.35, 0.35),
    );
    data.teachers.insert(Station::StudentRight, e);
    let e = place_teacher(
        &mut commands,
        teacher_a_teaching.clone(),
        (500., -230., 50.),
        (-0.4, 0.4),
    );
    data.teachers.insert(Station::StudentCenter, e);
    let e = place_teacher(
        &mut commands,
        teacher_a_teaching.clone(),
        (65., -230., 50.),
        (-0.4, 0.4),
    );
    data.teachers.insert(Station::StudentLeft, e);
    let e = place_teacher(
        &mut commands,
        teacher_a_protecting,
        (-220., 260., 12.),
        (0.38, 0.38),
    );
    data.teachers.insert(Station::Portal, e);
    let e = place_teacher(
        &mut commands,
        teacher_a_cooking,
        (770., 350., 12.),
        (0.25, 0.25),
    );
    data.teachers.insert(Station::Kitchen, e);

    let e = place_path(&mut commands, path_left_center, 78., -300., 1.);
    insert_data_path(&mut data, Station::StudentLeft, Station::StudentCenter, e);
    let e = place_path(&mut commands, path_center_right, 450., -300., 1.);
    insert_data_path(&mut data, Station::StudentCenter, Station::StudentRight, e);
    let e = place_path(&mut commands, path_right_kitchen, 850., 0., 1.);
    insert_data_path(&mut data, Station::StudentRight, Station::Kitchen, e);
    let e = place_path(&mut commands, path_kitchen_welcome, 513., 251., 1.);
    insert_data_path(&mut data, Station::Kitchen, Station::Welcome, e);
    let e = place_path(&mut commands, path_welcome_portal, -7., 212., 1.);
    insert_data_path(&mut data, Station::Welcome, Station::Portal, e);
    let e = place_path(&mut commands, path_portal_left, -312., -73., 1.);
    insert_data_path(&mut data, Station::Portal, Station::StudentLeft, e);
}

fn insert_data_path(data: &mut TeacherData, station1: Station, station2: Station, e: Entity) {
    //could as well check 1->2 and 2->1 if doesn't exist, but I don't think this takes much memory
    data.paths.insert((station1, station2), e);
    data.paths.insert((station2, station1), e);
}

fn place_path(
    commands: &mut Commands,
    texture: Handle<Image>,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3 {
                    x: pos_x,
                    y: pos_y,
                    z: 95.0,
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

fn place_teacher(
    commands: &mut Commands,
    texture: Handle<Image>,
    pos: (f32, f32, f32),
    scale: (f32, f32),
) -> Entity {
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

fn place_reaction(
    commands: &mut Commands,
    texture: Handle<Image>,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3 {
                    x: pos_x,
                    y: pos_y,
                    z: 100.0,
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

fn listen_teacher_moved(
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
    mut data: ResMut<TeacherData>,
) {
    for e in teacher_moved_events.read() {
        data.teacher_moved_event(e.teacher, Some(e.station_from), Some(e.station_to));
    }
}

fn listen_player_input(
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut data: ResMut<TeacherData>,
) {
    let mut dirty = false;

    for e in player_input_events.read() {
        if e.confirm_move {
            continue;
        }

        let direction_last = data.direction_last;
        data.direction_last = e.direction;
        if e.direction == Vec2::ZERO || e.direction != direction_last {
            //cleanup all path preview
            data.dirty = true;
            data.display_path_until
                .iter_mut()
                .for_each(|((teacher, _, _), (until, _))| {
                    if *teacher == e.teacher {
                        *until = 0.0;
                        dirty = true;
                    }
                });
        }

        if e.direction == Vec2::ZERO {
            continue;
        }

        let from = *data.teachers_position.get(&e.teacher).unwrap();
        if let Some(to) = possible_move(from, e.direction) {
            //don't override the reference if it already exists
            if let Some((until, _)) = data.display_path_until.get_mut(&(e.teacher, from, to)) {
                *until = f64::MAX;
                continue;
            }
            dirty = true;
            data.display_path_until
                .insert((e.teacher, from, to), (f64::MAX, Entity::PLACEHOLDER));
        } else {
            //for now, nothing when pointing toward the wrong direction.
            //We could display something here, or see also InvalidMoveEvent
        }
    }

    if dirty {
        data.dirty = true;
    }
}

fn listen_reactions(
    time: Res<Time>,
    mut invalid_action_station_events: EventReader<InvalidActionStationEvent>,
    mut taught_events: EventReader<TaughtEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut portal_observed_events: EventReader<PortalObservedEvent>,
    mut data: ResMut<TeacherData>,
) {
    let now = time.elapsed_seconds_f64();

    for e in invalid_action_station_events.read() {
        insert_reaction(&mut data, now, e.teacher, Reaction::Fail);
    }
    for e in taught_events.read() {
        insert_reaction(&mut data, now, e.teacher, Reaction::Short);
    }
    for e in student_welcomed_events.read() {
        insert_reaction(&mut data, now, e.teacher, Reaction::Short);
    }
    for e in portal_observed_events.read() {
        insert_reaction(&mut data, now, e.teacher, Reaction::Short);
    }
}

fn insert_reaction(data: &mut TeacherData, now: f64, teacher: Teacher, reaction: Reaction) {
    data.dirty = true;

    let dur = match reaction {
        Reaction::Fail => data.display_reaction_fail_duration_s,
        Reaction::Long => data.display_reaction_long_duration_s,
        Reaction::Short => data.display_reaction_short_duration_s,
    };
    let from = *data.teachers_position.get(&teacher).unwrap();
    data.display_reaction_until
        .insert((teacher, from, reaction), (now + dur, Entity::PLACEHOLDER));
}

fn listen_game_over(
    mut data: ResMut<TeacherData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.activated = false;
}

fn listen_reset(mut data: ResMut<TeacherData>, mut reset_game_events: EventReader<ResetGameEvent>) {
    if reset_game_events.read().last().is_some() {
        data.activated = true;
        data.dirty = true;

        data.teachers_position = HashMap::new();
        data.teachers_moved = Vec::new();

        //cleanup all teachers images
        for (s, _) in data.teachers.clone() {
            data.teachers_moved.push((Teacher::A, Some(s), None));
            data.teachers_moved.push((Teacher::B, Some(s), None));
        }
    }
}

fn draw(time: Res<Time>, mut data: ResMut<TeacherData>, mut query: Query<&mut Visibility>) {
    data.frame += Wrapping(1);
    //FIXME config
    if !data.dirty && data.frame.0 % 5 != 0 {
        return;
    }
    let now = time.elapsed_seconds_f64();

    //don't display the teacher where they were
    data.teachers_moved.iter().for_each(|(_, from, _)| {
        if let Some(s) = from {
            let e = *data.teachers.get(s).unwrap();
            if let Ok(mut visibility) = query.get_mut(e) {
                *visibility = Visibility::Hidden;
            }
        }
    });
    //display the teacher where they are now
    data.teachers_moved.iter().for_each(|(_, _, to)| {
        if let Some(s) = to {
            let e = *data.teachers.get(s).unwrap();
            if let Ok(mut visibility) = query.get_mut(e) {
                *visibility = Visibility::Visible;
            }
        }
    });
    data.teachers_moved.clear();

    for ((t, s, r), (until, e)) in data.display_reaction_until.clone() {
        if until > now {
            if let Ok(mut visibility) = query.get_mut(e) {
                *visibility = Visibility::Hidden;
            }
            data.display_reaction_until.remove(&(t, s, r));
            continue;
        }
        if e == Entity::PLACEHOLDER {
            let e = *data.reactions.get(&(s, r)).unwrap();
            if let Ok(mut visibility) = query.get_mut(e) {
                *visibility = Visibility::Visible;
            }
            continue;
        }
    }

    let mut keys_to_remove = vec![];
    let paths = data.paths.clone();
    for ((t, from, to), (until, e)) in data.display_path_until.iter_mut() {
        if *until < now {
            if let Ok(mut visibility) = query.get_mut(*e) {
                *visibility = Visibility::Hidden;
            }
            keys_to_remove.push((*t, *from, *to));
            continue;
        }
        if *e == Entity::PLACEHOLDER {
            *e = *paths.get(&(*from, *to)).unwrap();
            if let Ok(mut visibility) = query.get_mut(*e) {
                *visibility = Visibility::Visible;
            }
            continue;
        }
    }
    for key in keys_to_remove {
        data.display_path_until.remove(&key);
    }
}

pub struct TeacherViewPlugin;

impl Plugin for TeacherViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_player_input)
            .insert_resource(TeacherData::new())
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_teacher_moved)
            .add_systems(PreUpdate, listen_reactions)
            .add_systems(Update, draw)
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource)]
struct TeacherData {
    teachers: HashMap<Station, Entity>,
    paths: HashMap<(Station, Station), Entity>,
    reactions: HashMap<(Station, Reaction), Entity>,
    teachers_position: HashMap<Teacher, Station>,
    teachers_moved: Vec<(Teacher, Option<Station>, Option<Station>)>,
    activated: bool,
    dirty: bool,
    direction_last: Vec2,
    frame: Wrapping<i8>,
    display_reaction_until: HashMap<(Teacher, Station, Reaction), (f64, Entity)>,
    display_path_until: HashMap<(Teacher, Station, Station), (f64, Entity)>,
    display_reaction_short_duration_s: f64,
    display_reaction_long_duration_s: f64,
    display_reaction_fail_duration_s: f64,
}

impl TeacherData {
    pub fn new() -> Self {
        Self {
            teachers: HashMap::new(),
            paths: HashMap::new(),
            reactions: HashMap::new(),
            teachers_position: HashMap::new(),
            teachers_moved: Vec::new(),
            activated: false,
            dirty: false,
            direction_last: Vec2::ZERO,
            frame: Wrapping(0),
            display_reaction_until: HashMap::new(),
            display_path_until: HashMap::new(),
            display_reaction_fail_duration_s: 1.0,
            display_reaction_short_duration_s: 2.0,
            display_reaction_long_duration_s: 5.0,
        }
    }

    fn teacher_moved_event(
        &mut self,
        teacher: Teacher,
        from: Option<Station>,
        to: Option<Station>,
    ) {
        if to.is_some() {
            self.teachers_position.insert(teacher, to.unwrap());
        }
        //prepare for cleanup and draw
        self.teachers_moved.push((teacher, from, to));
        //No need to display the paths or reaction
        self.display_path_until
            .iter_mut()
            .for_each(|((t, _, _), (until, _))| {
                if teacher == *t {
                    *until = 0.;
                    self.dirty = true;
                }
            });
        //No need to display the reaction
        self.display_reaction_until
            .iter_mut()
            .for_each(|((t, _, _), (until, _))| {
                if teacher == *t {
                    *until = 0.;
                    self.dirty = true;
                }
            });
    }
}
