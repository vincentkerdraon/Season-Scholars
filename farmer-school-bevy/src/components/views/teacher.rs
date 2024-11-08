use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::controllers::moves::possible_move;
use crate::components::controllers::teacher_tired::TeacherTired;
use crate::model::config::Config;

use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::portal::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;

const DISPLAY_REACTION_FAIL_DURATION_S: f64 = 0.5;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<TeacherData>,
) {
    let reaction_fail = asset_server.load(config.base_path.join("images/ready/Reactions/fail.png"));
    let reaction_success_long = asset_server.load(
        config
            .base_path
            .join("images/ready/Reactions/successLong.png"),
    );
    let reaction_success_short = asset_server.load(
        config
            .base_path
            .join("images/ready/Reactions/successShort.png"),
    );

    let teacher_a_teaching = asset_server.load(
        config
            .base_path
            .join("images/ready/Teacher/TeacherA/TeacherATeachingWelcoming.png"),
    );
    let teacher_b_teaching = asset_server.load(
        config
            .base_path
            .join("images/ready/Teacher/TeacherB/TeacherBTeaching.png"),
    );
    let teacher_b_welcoming = asset_server.load(
        config
            .base_path
            .join("images/ready/Teacher/TeacherB/TeacherBWelcoming.png"),
    );
    let teacher_a_protecting = asset_server.load(
        config
            .base_path
            .join("images/ready/Teacher/TeacherA/teacherAProtecting.png"),
    );
    let teacher_b_protecting = asset_server.load(
        config
            .base_path
            .join("images/ready/Teacher/TeacherB/teacherBProtecting.png"),
    );
    let teacher_a_cooking = asset_server.load(
        config
            .base_path
            .join("images/ready/Cooking/cookingWithTeacherA.png"),
    );
    let teacher_b_cooking = asset_server.load(
        config
            .base_path
            .join("images/ready/Cooking/cookingWithTeacherB.png"),
    );

    let mut place_teacher_and_reactions =
        |station: Station,
         teacher_a: Handle<Image>,
         teacher_b: Handle<Image>,
         teacher_pos: (f32, f32, f32),
         teacher_scale: (f32, f32),
         reaction_pos: (f32, f32),
         reaction_scale: f32| {
            let e = place_reaction(
                &mut commands,
                reaction_fail.clone(),
                reaction_pos,
                reaction_scale,
            );
            data.reactions.insert((station, Reaction::Fail), e);
            let e = place_reaction(
                &mut commands,
                reaction_success_long.clone(),
                reaction_pos,
                reaction_scale,
            );
            data.reactions.insert((station, Reaction::Long), e);
            let e = place_reaction(
                &mut commands,
                reaction_success_short.clone(),
                reaction_pos,
                reaction_scale,
            );
            data.reactions.insert((station, Reaction::Short), e);

            let e = place_teacher(&mut commands, teacher_a.clone(), teacher_pos, teacher_scale);
            data.teachers.insert(station, (e, teacher_a, teacher_b));
        };

    place_teacher_and_reactions(
        Station::Welcome,
        teacher_a_teaching.clone(),
        teacher_b_welcoming.clone(),
        (450., 360., 10.),
        (-0.15, 0.15),
        (450., 450.),
        1.,
    );
    place_teacher_and_reactions(
        Station::StudentRight,
        teacher_a_teaching.clone(),
        teacher_b_teaching.clone(),
        (630., -230., 50.),
        (0.4, 0.4),
        (630., -100.),
        1.,
    );
    place_teacher_and_reactions(
        Station::StudentCenter,
        teacher_a_teaching.clone(),
        teacher_b_teaching.clone(),
        (65., -230., 50.),
        (0.4, 0.4),
        (65., -100.),
        1.,
    );
    place_teacher_and_reactions(
        Station::StudentLeft,
        teacher_a_teaching.clone(),
        teacher_b_teaching.clone(),
        (-440., -230., 50.),
        (0.4, 0.4),
        (-440., -100.),
        1.,
    );
    place_teacher_and_reactions(
        Station::Portal,
        teacher_a_protecting,
        teacher_b_protecting,
        (-220., 260., 12.),
        (0.4, 0.4),
        (-220., 260.),
        1.,
    );
    place_teacher_and_reactions(
        Station::Kitchen,
        teacher_a_cooking,
        teacher_b_cooking,
        (770., 300., 12.),
        (0.25, 0.25),
        (770., 350.),
        1.,
    );
}

fn load_resources_path(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<TeacherData>,
) {
    let path_left_center = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/ST_COL_LEFT_TO_ST_COL_CENTER.png"),
    );
    let path_center_right = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/ST_COL_RIGHT_TO_ST_COL_CENTER.png"),
    );
    let path_right_kitchen = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/ST_COL_RIGHT_TO_COOKING.png"),
    );
    let path_kitchen_welcome = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/WELCOME_TO_COOKING.png"),
    );
    let path_welcome_portal = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/WELCOME_TO_DOOR.png"),
    );
    let path_portal_left = asset_server.load(
        config
            .base_path
            .join("images/ready/Path/ST_COL_LEFT_TO_WINDOWS.png"),
    );

    let e = place_path(&mut commands, path_left_center, (-210., -490., 95.), 1.);
    insert_data_path(&mut data, Station::StudentLeft, Station::StudentCenter, e);
    let e = place_path(&mut commands, path_center_right, (250., -490., 95.), 1.);
    insert_data_path(&mut data, Station::StudentCenter, Station::StudentRight, e);
    let e = place_path(&mut commands, path_right_kitchen, (940., 0., 95.), 1.);
    insert_data_path(&mut data, Station::StudentRight, Station::Kitchen, e);
    let e = place_path(&mut commands, path_kitchen_welcome, (513., 251., 95.), 1.);
    insert_data_path(&mut data, Station::Kitchen, Station::Welcome, e);
    let e = place_path(&mut commands, path_welcome_portal, (-7., 212., 11.), 1.);
    insert_data_path(&mut data, Station::Welcome, Station::Portal, e);
    let e = place_path(&mut commands, path_portal_left, (-300., 41., 95.), 1.);
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
    pos: (f32, f32, f32),
    scale: f32,
) -> Entity {
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
            texture,
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
    pos: (f32, f32),
    scale: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3 {
                    x: pos.0,
                    y: pos.1,
                    z: 200.0,
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
    if !data.component_ready.listen_data_events {
        teacher_moved_events.clear();
        return;
    }

    for e in teacher_moved_events.read() {
        data.teacher_moved_event(e.teacher, Some(e.station_from), Some(e.station_to));
    }
}

fn listen_player_input(
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut data: ResMut<TeacherData>,
) {
    if !data.component_ready.listen_player_input {
        player_input_events.clear();
        return;
    }
    let mut dirty = false;

    for e in player_input_events.read() {
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

        if let Some(from) = data.teachers_position.get(&e.teacher).cloned() {
            if let Some((to, _)) = possible_move(from, e.direction) {
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
    }

    if dirty {
        data.dirty = true;
    }
}

fn listen_reactions(
    time: Res<Time>,
    mut data: ResMut<TeacherData>,
    mut invalid_action_station_events: EventReader<InvalidActionStationEvent>,
    mut teacher_ate_events: EventReader<TeacherAteEvent>,
    mut cooked_events: EventReader<CookedEvent>,
    mut observe_portal_events: EventReader<ObservePortalEvent>,
    mut portal_fixed_events: EventReader<PortalFixedEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut taught_events: EventReader<TaughtEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut recruit_student_events: EventReader<RecruitStudentEvent>,
) {
    if !data.component_ready.listen_data_events {
        return;
    }

    let now = time.elapsed_seconds_f64();
    let mut insert_reaction = |teacher: Teacher, reaction: Reaction| {
        data.dirty = true;
        if let Some((short, long)) = data.teacher_tired.get(&teacher) {
            let dur = match reaction {
                Reaction::Fail => DISPLAY_REACTION_FAIL_DURATION_S,
                Reaction::Long => long,
                Reaction::Short => short,
            };
            if let Some(station) = data.teachers_position.get(&teacher).cloned() {
                data.display_reaction_until.insert(
                    (teacher, station, reaction),
                    (now + dur, Entity::PLACEHOLDER),
                );
            }
        }
    };

    for e in invalid_action_station_events.read() {
        insert_reaction(e.teacher, Reaction::Fail);
    }
    for e in teacher_ate_events.read() {
        insert_reaction(e.teacher, Reaction::Short);
    }
    for e in cooked_events.read() {
        insert_reaction(e.teacher, Reaction::Long);
    }
    for e in observe_portal_events.read() {
        insert_reaction(e.teacher, Reaction::Short);
    }
    for e in portal_fixed_events.read() {
        insert_reaction(e.teacher, Reaction::Long);
    }
    for e in graduated_events.read() {
        insert_reaction(e.teacher, Reaction::Long);
    }
    for e in taught_events.read() {
        insert_reaction(e.teacher, Reaction::Short);
    }
    for e in student_welcomed_events.read() {
        insert_reaction(e.teacher, Reaction::Short);
    }
    for e in recruit_student_events.read() {
        insert_reaction(e.teacher, Reaction::Long);
    }
}

fn listen_game_over(
    mut data: ResMut<TeacherData>,
    mut game_over_events: EventReader<GameOverEvent>,
) {
    if game_over_events.read().last().is_none() {
        return;
    }
    data.component_ready = ComponentReady {
        listen_data_events: false,
        listen_player_input: false,
    };
}

fn listen_reset(
    mut data: ResMut<TeacherData>,
    mut reset_game_step1_events: EventReader<ResetGameStep1Event>,
    mut reset_game_step2_events: EventReader<ResetGameStep2Event>,
    mut reset_game_step3_events: EventReader<ResetGameStep3Event>,
) {
    if let Some(e) = reset_game_step1_events.read().last() {
        data.component_ready.listen_data_events = true;
        data.teachers_position = HashMap::new();
        data.teacher_tired = TeacherTired::new(&e.teachers);
    }

    if let Some(_e) = reset_game_step2_events.read().last() {
        data.dirty = true;

        data.teachers_moved = Vec::new();
        //cleanup all teachers images
        for (s, _) in data.teachers.clone() {
            data.teachers_moved.push((Teacher::A, Some(s), None));
            data.teachers_moved.push((Teacher::B, Some(s), None));
        }
    }
    if let Some(_e) = reset_game_step3_events.read().last() {
        data.component_ready.listen_player_input = true;
    }
}

fn listen_events_teacher_tired(
    time: Res<Time>,
    mut data: ResMut<TeacherData>,
    mut teacher_tired_events: EventReader<TeacherTiredEvent>,
) {
    for e in teacher_tired_events.read() {
        let now = time.elapsed_seconds_f64();
        data.teacher_tired
            .update(now, &e.teacher, e.short_action, e.long_action)
    }
}

fn draw(
    time: Res<Time>,
    mut data: ResMut<TeacherData>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
) {
    if !data.component_ready.listen_data_events {
        return;
    }

    if !data.dirty {
        return;
    }
    let now = time.elapsed_seconds_f64();

    //don't display the teacher where they were
    data.teachers_moved.iter().for_each(|(_, from, _)| {
        if let Some(s) = from {
            let (e, _, _) = *data.teachers.get(s).unwrap();
            if let Ok((_, mut visibility)) = query.get_mut(e) {
                *visibility = Visibility::Hidden;
            }
        }
    });
    //display the teacher where they are now
    let updates: Vec<_> = data
        .teachers_moved
        .iter()
        .filter_map(|(teacher, _, to)| {
            if let Some(s) = to {
                if let Some((e, teacher_a, teacher_b)) = data.teachers.get(s) {
                    return Some((*e, teacher, teacher_a.clone(), teacher_b.clone()));
                }
            }
            None
        })
        .collect();

    for (e, teacher, teacher_a, teacher_b) in updates {
        if let Ok((mut texture, mut visibility)) = query.get_mut(e) {
            *visibility = Visibility::Visible;
            match teacher {
                Teacher::A => *texture = teacher_a,
                Teacher::B => *texture = teacher_b,
            }
        }
    }

    data.teachers_moved.clear();

    let mut keys_to_remove: Vec<(Teacher, Station, Reaction)> = Vec::new();
    let data_reactions = data.reactions.clone();
    for ((t, s, r), (until, e)) in data.display_reaction_until.iter_mut() {
        if *until < now {
            if let Ok((_, mut visibility)) = query.get_mut(*e) {
                *visibility = Visibility::Hidden;
            }
            keys_to_remove.push((*t, *s, *r));

            continue;
        }
        if *e == Entity::PLACEHOLDER {
            *e = *data_reactions.get(&(*s, *r)).unwrap();
            if let Ok((_, mut visibility)) = query.get_mut(*e) {
                *visibility = Visibility::Visible;
            }
            continue;
        }
    }
    for key in keys_to_remove {
        data.display_reaction_until.remove(&key);
    }

    let mut keys_to_remove = Vec::new();
    let paths = data.paths.clone();
    for ((t, from, to), (until, e)) in data.display_path_until.iter_mut() {
        if *until < now {
            if let Ok((_, mut visibility)) = query.get_mut(*e) {
                *visibility = Visibility::Hidden;
            }
            keys_to_remove.push((*t, *from, *to));
            continue;
        }
        if *e == Entity::PLACEHOLDER {
            *e = *paths.get(&(*from, *to)).unwrap();
            if let Ok((_, mut visibility)) = query.get_mut(*e) {
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
        app.insert_resource(TeacherData::new())
            .add_systems(Startup, load_resources)
            .add_systems(Startup, load_resources_path)
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_teacher_tired)
            .add_systems(PreUpdate, listen_teacher_moved)
            .add_systems(PreUpdate, listen_player_input)
            .add_systems(PreUpdate, listen_reactions)
            .add_systems(Update, draw);
    }
}

#[derive(Resource)]
struct TeacherData {
    teachers: HashMap<Station, (Entity, Handle<Image>, Handle<Image>)>,
    paths: HashMap<(Station, Station), Entity>,
    reactions: HashMap<(Station, Reaction), Entity>,
    teachers_position: HashMap<Teacher, Station>,
    teachers_moved: Vec<(Teacher, Option<Station>, Option<Station>)>,
    teacher_tired: TeacherTired,

    component_ready: ComponentReady,
    dirty: bool,
    direction_last: Vec2,
    display_reaction_until: HashMap<(Teacher, Station, Reaction), (f64, Entity)>,
    display_path_until: HashMap<(Teacher, Station, Station), (f64, Entity)>,
}

impl TeacherData {
    pub fn new() -> Self {
        Self {
            teachers: HashMap::new(),
            paths: HashMap::new(),
            reactions: HashMap::new(),
            teachers_position: HashMap::new(),
            teachers_moved: Vec::new(),
            teacher_tired: TeacherTired::default(),
            dirty: false,
            direction_last: Vec2::ZERO,
            display_reaction_until: HashMap::new(),
            display_path_until: HashMap::new(),
            component_ready: ComponentReady::default(),
        }
    }

    fn teacher_moved_event(
        &mut self,
        teacher: Teacher,
        from: Option<Station>,
        to: Option<Station>,
    ) {
        if let Some(to) = to {
            self.teachers_position.insert(teacher, to);
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
