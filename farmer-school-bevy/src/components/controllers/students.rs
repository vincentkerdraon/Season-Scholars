use super::teacher_busy::TeacherBusy;
use super::teacher_tired::TeacherTired;
use crate::model::config::Config;

use crate::model::overlord::*;
use crate::model::player_input::*;
use crate::model::season::*;
use crate::model::students::*;
use crate::model::teacher::*;
use crate::model::welcome::*;
use bevy::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

fn listen_move(
    mut data: ResMut<StudentsData>,
    mut teacher_moved_events: EventReader<TeacherMovedEvent>,
) {
    for e in teacher_moved_events.read() {
        data.teacher_busy.moved(e);
    }
}

fn listen_game_over(
    mut data: ResMut<StudentsData>,
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

fn listen_welcomed(
    mut data: ResMut<StudentsData>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut students_seated_events: EventWriter<StudentsSeatedEvent>,
) {
    let mut dirty = false;
    for _ in student_welcomed_events.read() {
        data.create_student();
        dirty = true;
    }

    if !dirty {
        return;
    }

    let emit = StudentsSeatedEvent {
        students: data.students.values().cloned().collect(),
    };
    debug!("{:?}", emit);
    students_seated_events.send(emit);
}

fn listen_reset(
    config: Res<Config>,
    mut data: ResMut<StudentsData>,
    mut reset_game_step1_events: EventReader<ResetGameStep1Event>,
    mut reset_game_step2_events: EventReader<ResetGameStep2Event>,
    mut reset_game_step3_events: EventReader<ResetGameStep3Event>,
    mut students_seated_events: EventWriter<StudentsSeatedEvent>,
) {
    if let Some(e) = reset_game_step1_events.read().last() {
        data.students_rows_nb = config.students_rows_nb;
        data.teacher_busy = TeacherBusy::new(&e.teachers);
        data.teacher_tired = TeacherTired::new(&e.teachers);
        data.component_ready.listen_data_events = true;

        data.reset();
        for _ in 0..config.students_init {
            if data.create_student().is_none() {
                panic!();
            }
        }
    }
    if let Some(_e) = reset_game_step2_events.read().last() {
        let emit = StudentsSeatedEvent {
            students: data.students.values().cloned().collect(),
        };
        debug!("{:?}", emit);
        students_seated_events.send(emit);
    }
    if let Some(_e) = reset_game_step3_events.read().last() {
        data.component_ready.listen_player_input = true;
    }
}

fn listen_events_teacher_tired(
    time: Res<Time>,
    mut data: ResMut<StudentsData>,
    mut teacher_tired_events: EventReader<TeacherTiredEvent>,
) {
    for e in teacher_tired_events.read() {
        let now = time.elapsed_seconds_f64();
        data.teacher_tired
            .update(now, &e.teacher, e.short_action, e.long_action)
    }
}

fn listen_season(
    mut data: ResMut<StudentsData>,
    mut season_changed_events: EventReader<SeasonChangedEvent>,
) {
    if let Some(e) = season_changed_events.read().last() {
        data.season = e.season;
    }
}

fn listen_events_player_input(
    time: Res<Time>,
    mut data: ResMut<StudentsData>,
    mut player_input_events: EventReader<PlayerInputEvent>,
    mut move_teacher_events: EventWriter<MoveTeacherEvent>,
    mut graduate_events: EventWriter<GraduateEvent>,
    mut graduated_events: EventWriter<GraduatedEvent>,
    mut teach_events: EventWriter<TeachEvent>,
    mut taught_events: EventWriter<TaughtEvent>,
    mut invalid_action_station_events: EventWriter<InvalidActionStationEvent>,
    mut invalid_move_events: EventWriter<InvalidMoveEvent>,
) {
    if !data.component_ready.listen_player_input {
        player_input_events.clear();
        return;
    }

    let now = time.elapsed_seconds_f64();
    for e in player_input_events.read() {
        if let Some(station) = data.teacher_busy.station(e.teacher) {
            if !vec![
                Station::StudentLeft,
                Station::StudentCenter,
                Station::StudentRight,
            ]
            .contains(&station)
            {
                continue;
            }
            if data.teacher_busy.ready(e.teacher, now) != (true, true) {
                continue;
            }

            let col = station_to_student_col(station);

            if e.long_action {
                if let Some(graduate) = data.graduate(col) {
                    if let Some((_, long)) = data.teacher_tired.get(&e.teacher) {
                        data.teacher_busy.action(e.teacher, now, long);
                        let emit = GraduateEvent {
                            student_col: col,
                            teacher: e.teacher,
                        };
                        debug!("{:?}", emit);
                        graduate_events.send(emit);
                        let emit = GraduatedEvent {
                            knowledge: graduate.knowledge,
                            student_id: graduate.id,
                            students: data.students.values().cloned().collect(),
                            teacher: e.teacher,
                        };
                        debug!("{:?}", emit);
                        graduated_events.send(emit);
                    }
                } else {
                    let emit = InvalidActionStationEvent {
                        station,
                        teacher: e.teacher,
                    };
                    trace!("{:?}", emit);
                    invalid_action_station_events.send(emit);
                }
                continue;
            }

            if e.short_action {
                if let Some(season) = data.teach(col) {
                    if let Some((short, _)) = data.teacher_tired.get(&e.teacher) {
                        data.teacher_busy.action(e.teacher, now, short);
                        let emit = TeachEvent {
                            teacher: e.teacher,
                            student_col: col,
                        };
                        debug!("{:?}", emit);
                        teach_events.send(emit);
                        let emit = TaughtEvent {
                            knowledge: season,
                            student_col: col,
                            students: data.students.values().cloned().collect(),
                            teacher: e.teacher,
                        };
                        debug!("{:?}", emit);
                        taught_events.send(emit);
                    }
                } else {
                    let emit = InvalidActionStationEvent {
                        station,
                        teacher: e.teacher,
                    };
                    trace!("{:?}", emit);
                    invalid_action_station_events.send(emit);
                }
                continue;
            }

            if e.direction != Vec2::ZERO {
                if let Some(to) = data
                    .teacher_busy
                    .possible_move(e.teacher, station, e.direction)
                {
                    let emit = MoveTeacherEvent {
                        station_from: station,
                        station_to: to,
                        teacher: e.teacher,
                    };
                    debug!("{:?}", emit);
                    move_teacher_events.send(emit);
                } else {
                    let emit = InvalidMoveEvent {
                        station,
                        teacher: e.teacher,
                    };
                    trace!("{:?}", emit);
                    invalid_move_events.send(emit);
                }
                continue;
            }
        }
    }
}

pub struct StudentsControllerPlugin;

impl Plugin for StudentsControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraduateEvent>()
            .add_event::<GraduatedEvent>()
            .add_event::<TeachEvent>()
            .add_event::<TaughtEvent>()
            .add_event::<StudentsSeatedEvent>()
            .insert_resource(StudentsData { ..default() })
            .add_systems(PreUpdate, listen_reset)
            .add_systems(PreUpdate, listen_game_over)
            .add_systems(PreUpdate, listen_events_teacher_tired)
            .add_systems(PreUpdate, listen_move)
            .add_systems(PreUpdate, listen_welcomed)
            .add_systems(PreUpdate, listen_season)
            .add_systems(PreUpdate, listen_events_player_input);
    }
}

#[derive(Resource, Default)]
struct StudentsData {
    component_ready: ComponentReady,
    students_rows_nb: i8,
    students: HashMap<StudentId, Student>,
    last_id: i64,
    teacher_busy: TeacherBusy,
    season: Season,
    teacher_tired: TeacherTired,
}

impl StudentsData {
    // fn row_has_student(&mut self, col: StudentCol) -> bool {
    //     return self.students.iter().any(|(_, student)| student.col != col);
    // }

    fn reset(&mut self) {
        self.students.clear();
    }

    fn teach(&mut self, col: StudentCol) -> Option<Season> {
        let mut res: Option<Season> = None;
        self.students.iter_mut().for_each(|(_, student)| {
            if student.col != col {
                return;
            }
            if student.knowledge.len() >= 3 {
                return;
            }
            student.knowledge.push(self.season);
            res = Some(self.season);
        });
        res
    }

    fn graduate(&mut self, col: StudentCol) -> Option<Student> {
        //find the first student of the col
        //move all others students in the col toward the front

        let mut res: Option<Student> = None;
        let mut keys_to_remove: Vec<StudentId> = Vec::new();
        for i in 0..=self.students_rows_nb {
            self.students
                .iter_mut()
                .filter(|(_, s)| s.col == col && s.row == i)
                .for_each(|(_, student)| {
                    if i == 0 {
                        keys_to_remove.push(student.id);
                        res = Some(student.clone());
                        return;
                    }
                    student.row -= 1;
                });
        }
        for key in keys_to_remove {
            self.students.remove(&key);
        }

        res
    }

    fn create_student(&mut self) -> Option<Student> {
        self.last_id += 1;
        if let Some((col, row)) = self.find_available_desk() {
            let s = Student {
                id: self.last_id,
                col,
                row,
                knowledge: Vec::new(),
            };
            self.students.insert(s.id, s.clone());
            return Some(s);
        }
        None
    }

    fn find_available_desk(&self) -> Option<(StudentCol, StudentRow)> {
        if self.students.len() == 9 {
            return None;
        }
        if self.students.is_empty() {
            return Some((StudentCol::Right, 0));
        }

        let mut col_count: HashMap<StudentCol, usize> = HashMap::new();

        for student in self.students.values() {
            *col_count.entry(student.col).or_insert(0) += 1;
        }

        let mut min = usize::MAX;
        let mut col_min = StudentCol::Center;
        for col in StudentCol::iter() {
            if let Some(val) = col_count.get(&col) {
                if *val < min {
                    min = *val;
                    col_min = col;
                }
            } else {
                min = 0;
                col_min = col;
                break;
            }
        }

        Some((col_min, (min) as StudentRow))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut data = StudentsData {
            ..Default::default()
        };

        fn check_col_row(student: &Student, col: StudentCol, row: StudentRow) {
            assert_eq!(student.row, row, "Wrong row");
            assert_eq!(student.col, col, "Wrong col");
        }

        for i in 1..=10 {
            let student = data.create_student();
            if i <= 9 {
                assert!(
                    student.is_some(),
                    "Student should be created when the desk is available."
                );
                let student = student.unwrap();
                assert_eq!(student.id, i, "Student ID should be incremented correctly.");
                match i {
                    1 => check_col_row(&student, StudentCol::Right, 0),
                    2 => check_col_row(&student, StudentCol::Left, 0),
                    3 => check_col_row(&student, StudentCol::Center, 0),
                    4 => check_col_row(&student, StudentCol::Left, 1),
                    5 => check_col_row(&student, StudentCol::Center, 1),
                    6 => check_col_row(&student, StudentCol::Right, 1),
                    7 => check_col_row(&student, StudentCol::Left, 2),
                    8 => check_col_row(&student, StudentCol::Center, 2),
                    9 => check_col_row(&student, StudentCol::Right, 2),
                    _ => panic!(),
                }

                assert!(
                    data.students.contains_key(&student.id),
                    "Student should be added to the students HashMap."
                );
            } else {
                assert!(
                    student.is_none(),
                    "No student should be created when no desks are available."
                );
            }
        }
    }
}
