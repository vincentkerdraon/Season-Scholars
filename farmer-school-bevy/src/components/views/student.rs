use std::collections::HashMap;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::model::config::Config;
use crate::model::season::Season;
use crate::model::students::*;

const STUDENTS_IMG_CENTER_NB: i8 = 5;
const STUDENTS_IMG_SIDE_NB: i8 = 6;
const STUDENTS_KNOWLEDGE_MAX: i8 = 3;

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<StudentData>,
) {
    //images name start at 1
    for i in 1..=STUDENTS_IMG_CENTER_NB {
        data.students_center.push((
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Students/c{}.png", i)),
            ),
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Students/c{}_empty.png", i)),
            ),
        ));
    }

    //images name start at 1
    for i in 1..=STUDENTS_IMG_SIDE_NB {
        data.students_side.push((
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Students/s{}.png", i)),
            ),
            asset_server.load(
                config
                    .base_path
                    .join(format!("images/ready/Students/s{}_empty.png", i)),
            ),
        ));
    }

    data.desk_free_center =
        asset_server.load(config.base_path.join("images/ready/Students/c0.png"));
    data.desk_free_side = asset_server.load(config.base_path.join("images/ready/Students/s0.png"));

    let s0 = 1.4;
    let s1 = 1.0;
    let s2 = 0.8;
    let z0 = 23.;
    let z1 = 22.;
    let z2 = 21.;
    let t = data.desk_free_side.clone();
    data.desks.insert(
        (StudentCol::Left, 0),
        place_desk(&mut commands, t.clone(), (-210., -170., z0), (s0, s0)),
    );
    data.desks.insert(
        (StudentCol::Left, 1),
        place_desk(&mut commands, t.clone(), (-50., 60., z1), (s1, s1)),
    );
    data.desks.insert(
        (StudentCol::Left, 2),
        place_desk(&mut commands, t.clone(), (30., 230., z2), (s2, s2)),
    );

    data.desks.insert(
        (StudentCol::Right, 0),
        place_desk(&mut commands, t.clone(), (800., -180., z0), (-s0, s0)),
    );
    data.desks.insert(
        (StudentCol::Right, 1),
        place_desk(&mut commands, t.clone(), (680., 60., z1), (-s1, s1)),
    );
    data.desks.insert(
        (StudentCol::Right, 2),
        place_desk(&mut commands, t.clone(), (600., 230., z2), (-s2, s2)),
    );

    let t = data.desk_free_center.clone();
    data.desks.insert(
        (StudentCol::Center, 0),
        place_desk(&mut commands, t.clone(), (260., -170., z0), (s0, s0)),
    );
    data.desks.insert(
        (StudentCol::Center, 1),
        place_desk(&mut commands, t.clone(), (310., 50., z1), (s1, s1)),
    );
    data.desks.insert(
        (StudentCol::Center, 2),
        place_desk(&mut commands, t.clone(), (330., 220., z2), (s2, s2)),
    );

    data.seasons.insert(
        Season::Spring,
        asset_server.load(config.base_path.join("images/ready/Harvest/HarvestA0.png")),
    );
    data.seasons.insert(
        Season::Summer,
        asset_server.load(config.base_path.join("images/ready/Harvest/HarvestC0.png")),
    );
    data.seasons.insert(
        Season::Autumn,
        asset_server.load(config.base_path.join("images/ready/Harvest/HarvestM0.png")),
    );
    data.seasons.insert(
        Season::Winter,
        asset_server.load(config.base_path.join("images/ready/Harvest/HarvestL0.png")),
    );

    let t = data.seasons.get(&Season::Autumn).unwrap().clone();
    data.knowledge.insert(
        (StudentCol::Left, 0),
        place_knowledge(&mut commands, t.clone(), (-230., -230.), (-0.5, 0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Left, 1),
        place_knowledge(&mut commands, t.clone(), (-110., -225.), (0.5, -0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Left, 2),
        place_knowledge(&mut commands, t.clone(), (-200., -180.), (0.5, 0.5)),
    );

    data.knowledge.insert(
        (StudentCol::Center, 0),
        place_knowledge(&mut commands, t.clone(), (230., -230.), (0.5, -0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Center, 1),
        place_knowledge(&mut commands, t.clone(), (375., -225.), (0.5, -0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Center, 2),
        place_knowledge(&mut commands, t.clone(), (270., -190.), (-0.5, 0.5)),
    );

    data.knowledge.insert(
        (StudentCol::Right, 0),
        place_knowledge(&mut commands, t.clone(), (760., -230.), (0.5, -0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Right, 1),
        place_knowledge(&mut commands, t.clone(), (900., -215.), (0.5, -0.5)),
    );
    data.knowledge.insert(
        (StudentCol::Right, 2),
        place_knowledge(&mut commands, t.clone(), (800., -190.), (-0.5, -0.5)),
    );
}

fn place_knowledge(
    commands: &mut Commands,
    image: Handle<Image>,
    pos: (f32, f32),
    scale: (f32, f32),
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3 {
                    x: pos.0,
                    y: pos.1,
                    z: 26.,
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

fn place_desk(
    commands: &mut Commands,
    image: Handle<Image>,
    pos: (f32, f32, f32),
    scale: (f32, f32),
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: image,
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
            ..default()
        })
        .id()
}

fn listen_events(
    mut graduated_events: EventReader<GraduatedEvent>,
    mut taught_events: EventReader<TaughtEvent>,
    mut students_seated_events: EventReader<StudentsSeatedEvent>,
    mut data: ResMut<StudentData>,
) {
    // we could process all the graduated and cleanup our local cache,
    // OR we just cleanup every time. <-
    // for e in graduated_events.read() {
    //     data.graduate(e.student_id);
    //     data.refresh(&e.students);
    // }

    if let Some(e) = graduated_events.read().last() {
        data.refresh(&e.students);
    }
    if let Some(e) = taught_events.read().last() {
        data.refresh(&e.students);
    }
    if let Some(e) = students_seated_events.read().last() {
        data.refresh(&e.students);
    }
}

fn draw(
    config: Res<Config>,
    mut data: ResMut<StudentData>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility)>,
) {
    if !data.dirty {
        return;
    }
    data.dirty = false;

    for col in StudentCol::iter() {
        let mut texture_empty = data.desk_free_side.clone();
        if col == StudentCol::Center {
            texture_empty = data.desk_free_center.clone();
        }
        for row in 0..config.students_rows_nb {
            let e = *data.desks.get(&(col, row)).unwrap();
            if let Ok((mut texture_handle, _)) = query.get_mut(e) {
                //now we have the image ref for this {row;col}, let's find the texture

                let mut done = false;
                for student in &data.students {
                    if student.col != col || student.row != row {
                        continue;
                    }
                    if let Some((_, texture_index)) = data.mapping.get(&student.id) {
                        let mut students_images = &data.students_side;
                        if col == StudentCol::Center {
                            students_images = &data.students_center;
                        }
                        //only the first row displays their knowledge
                        //the others have the basic art
                        if row > 0 || student.knowledge.is_empty() {
                            *texture_handle =
                                students_images.get(*texture_index).unwrap().0.clone();
                            done = true;
                            break;
                        } else {
                            *texture_handle =
                                students_images.get(*texture_index).unwrap().1.clone();
                            done = true;
                            break;
                        }
                    }
                }
                if !done {
                    *texture_handle = texture_empty.clone();
                }
            }
        }
    }

    for col in StudentCol::iter() {
        let student = data
            .students
            .iter()
            .find(|student| student.col == col && student.row == 0);

        for knowledge_index in 0..STUDENTS_KNOWLEDGE_MAX {
            let mut found = false;
            let e: Entity = *data.knowledge.get(&(col, knowledge_index)).unwrap();
            let (mut texture_handle, mut visibility) = query.get_mut(e).unwrap();

            if let Some(student) = student {
                if let Some(s) = student.knowledge.get(knowledge_index as usize) {
                    found = true;
                    *visibility = Visibility::Visible;
                    *texture_handle = data.seasons.get(s).unwrap().clone();
                }
            }
            if !found {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub struct StudentViewPlugin;

impl Plugin for StudentViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StudentData::new())
            .add_systems(Startup, load_resources)
            .add_systems(PreUpdate, listen_events)
            .add_systems(Update, draw);
    }
}

type TextureIndex = usize;

#[derive(Resource, Default)]
struct StudentData {
    desks: HashMap<(StudentCol, StudentRow), Entity>,
    students_center: Vec<(Handle<Image>, Handle<Image>)>,
    students_side: Vec<(Handle<Image>, Handle<Image>)>,
    students_center_last_used_index: TextureIndex,
    students_side_last_used_index: TextureIndex,
    desk_free_center: Handle<Image>,
    desk_free_side: Handle<Image>,

    knowledge: HashMap<(StudentCol, i8), Entity>,
    seasons: HashMap<Season, Handle<Image>>,

    dirty: bool,
    mapping: HashMap<StudentId, (StudentCol, TextureIndex)>,
    students: Vec<Student>,
}

impl StudentData {
    fn new() -> Self {
        Self { ..default() }
    }

    fn refresh(&mut self, students: &Vec<Student>) {
        //cleanup self.mapping, that should be done on graduate but this is a safety.
        let mut keys_to_remove: Vec<i64> = Vec::new();
        for (s_id, _) in self.mapping.clone() {
            let mut found = false;
            for s in students {
                if s.id == s_id {
                    found = true;
                    break;
                }
            }
            if !found {
                keys_to_remove.push(s_id);
            }
        }
        for key in keys_to_remove {
            self.mapping.remove(&key);
        }

        //add new elements
        for s in students {
            if !self.mapping.contains_key(&s.id) {
                self.new_student(s.id, s.col);
            }
        }
        self.students.clone_from(students);
        self.dirty = true;
    }

    fn new_student(&mut self, id: StudentId, col: StudentCol) -> (TextureIndex, Handle<Image>) {
        fn insert(
            id: StudentId,
            col: StudentCol,
            mapping: &mut HashMap<i64, (StudentCol, usize)>,
            last_used_index: &mut usize,
            students_col: &[(Handle<Image>, Handle<Image>)],
        ) -> (TextureIndex, Handle<Image>) {
            *last_used_index += 1;
            if *last_used_index >= students_col.len() {
                *last_used_index = 0;
            }

            mapping.insert(id, (col, *last_used_index));
            return (
                *last_used_index,
                students_col.get(*last_used_index).unwrap().0.clone(),
            );
        }

        if col == StudentCol::Center {
            return insert(
                id,
                col,
                &mut self.mapping,
                &mut self.students_center_last_used_index,
                &self.students_center,
            );
        }
        insert(
            id,
            col,
            &mut self.mapping,
            &mut self.students_side_last_used_index,
            &self.students_side,
        )
    }

    // fn graduate(&mut self, id: StudentId) {
    //     self.mapping.remove(&id);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_student() {
        let mut test_struct = StudentData {
            students_center_last_used_index: 0,
            students_center: vec![
                (Handle::weak_from_u128(0), Handle::weak_from_u128(0)),
                (Handle::weak_from_u128(0), Handle::weak_from_u128(0)),
                (Handle::weak_from_u128(0), Handle::weak_from_u128(0)),
            ],
            ..default()
        };

        test_struct.new_student(1, StudentCol::Center);
        assert_eq!(test_struct.students_center_last_used_index, 1);

        test_struct.new_student(2, StudentCol::Center);
        assert_eq!(test_struct.students_center_last_used_index, 2);

        test_struct.new_student(3, StudentCol::Center);
        assert_eq!(test_struct.students_center_last_used_index, 0); // It should wrap around
    }
}
