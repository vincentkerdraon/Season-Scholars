use std::collections::HashMap;
use std::vec;

use bevy::prelude::*;

use crate::components::controllers::teacher_tired::TeacherTired;
use crate::model::config::Config;

use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::portal::*;
use crate::model::students::*;
use crate::model::teacher::TeacherTiredEvent;
use crate::model::welcome::*;
use bevy::audio::{AudioSource, PlaybackMode, Volume};

fn load_resources(
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<SoundData>,
) {
    let track = asset_server.load(config.base_path.join("sounds/ready/track_1.ogg"));
    data.tracks.push((track.clone(), 20., 0.));
    let track = asset_server.load(config.base_path.join("sounds/ready/track_2.ogg"));
    data.tracks.push((track.clone(), 22., 0.));

    data.monsters
        .push(asset_server.load(config.base_path.join("sounds/ready/monster_1.ogg")));
    let m1 = asset_server.load(config.base_path.join("sounds/ready/fail_short_1.ogg"));
    data.reactions.insert(Reaction::Fail, vec![(m1, 0.574)]);
    let m1 = asset_server.load(config.base_path.join("sounds/ready/success_long_1.ogg"));
    let m2 = asset_server.load(config.base_path.join("sounds/ready/success_long_2.ogg"));
    data.reactions
        .insert(Reaction::Long, vec![(m1, 5.668), (m2, 5.015)]);

    let m1 = asset_server.load(config.base_path.join("sounds/ready/success_short_1.ogg"));
    let m2 = asset_server.load(config.base_path.join("sounds/ready/success_short_2.ogg"));
    data.reactions
        .insert(Reaction::Short, vec![(m1, 0.600), (m2, 0.522)]);
}

fn play_sound(commands: &mut Commands, h: Handle<AudioSource>, volume: f32, speed: f32) {
    //spawn a new one each time, to be able to overlap.
    //not great for perf
    commands.spawn((AudioBundle {
        source: h,
        settings: PlaybackSettings {
            mode: PlaybackMode::Remove,
            paused: false,
            volume: Volume::new(volume),
            speed,
            ..default()
        },
    },));
}

fn play_track(
    time: Res<Time>,
    mut commands: Commands,
    config: Res<Config>,
    mut data: ResMut<SoundData>,
) {
    let now = time.elapsed_seconds_f64();
    let (_, duration, started_at) = data
        .tracks
        .get(data.tracks_last_used_index as usize)
        .unwrap();
    if *started_at + *duration + config.track_break_s > now {
        return;
    }
    data.tracks_last_used_index += 1;
    if data.tracks_last_used_index as usize == data.tracks.len() {
        data.tracks_last_used_index = 0;
    }
    let tracks_last_used_index = data.tracks_last_used_index as usize;
    let (t, _, started_at) = data.tracks.get_mut(tracks_last_used_index).unwrap();
    play_sound(&mut commands, t.clone(), 0.3, 1.);
    *started_at = now;
}

fn listen_reactions(
    mut commands: Commands,
    mut invalid_action_station_events: EventReader<InvalidActionStationEvent>,
    mut teacher_ate_events: EventReader<TeacherAteEvent>,
    mut cooked_events: EventReader<CookedEvent>,
    mut observe_portal_events: EventReader<ObservePortalEvent>,
    mut portal_fixed_events: EventReader<PortalFixedEvent>,
    mut graduated_events: EventReader<GraduatedEvent>,
    mut taught_events: EventReader<TaughtEvent>,
    mut student_welcomed_events: EventReader<StudentWelcomedEvent>,
    mut recruit_student_events: EventReader<RecruitStudentEvent>,
    mut data: ResMut<SoundData>,
) {
    let mut play_reaction = |teacher: Teacher, reaction: Reaction| {
        let (h, duration) = data
            .reactions
            .get(&reaction)
            .unwrap()
            .first()
            .unwrap()
            .clone();

        let (short, long) = data.teacher_tired.get(&teacher).unwrap();
        let mut speed: f32 = 1.;
        match reaction {
            Reaction::Short => speed = (duration / short) as f32,
            Reaction::Long => speed = (duration / long) as f32,
            Reaction::Fail => {}
        }
        trace!(
            "play reaction {:?} (short={}s,long={}s), original={}s => speed={}",
            reaction,
            short,
            long,
            duration,
            speed
        );

        play_sound(&mut commands, h, 1., speed);
    };

    if let Some(e) = invalid_action_station_events.read().last() {
        play_reaction(e.teacher, Reaction::Fail);
    }
    if let Some(e) = teacher_ate_events.read().last() {
        play_reaction(e.teacher, Reaction::Short);
    }
    if let Some(e) = cooked_events.read().last() {
        play_reaction(e.teacher, Reaction::Long);
    }
    if let Some(e) = observe_portal_events.read().last() {
        play_reaction(e.teacher, Reaction::Short);
    }
    if let Some(e) = portal_fixed_events.read().last() {
        play_reaction(e.teacher, Reaction::Long);
    }
    if let Some(e) = graduated_events.read().last() {
        play_reaction(e.teacher, Reaction::Long);
    }
    if let Some(e) = taught_events.read().last() {
        play_reaction(e.teacher, Reaction::Short);
    }
    if let Some(e) = student_welcomed_events.read().last() {
        play_reaction(e.teacher, Reaction::Short);
    }
    if let Some(e) = recruit_student_events.read().last() {
        play_reaction(e.teacher, Reaction::Long);
    }
}

fn listen_monster_attack(
    mut commands: Commands,
    mut portal_attacked_events: EventReader<PortalAttackedEvent>,
    mut data: ResMut<SoundData>,
) {
    if portal_attacked_events.read().last().is_some() {
        data.monsters_last_used_index += 1;
        if data.monsters_last_used_index as usize == data.monsters.len() {
            data.monsters_last_used_index = 0;
        }
        let h = data
            .monsters
            .get(data.monsters_last_used_index as usize)
            .unwrap()
            .clone();
        play_sound(&mut commands, h, 1.0, 1.);
    }
}

fn listen_events_teacher_tired(
    time: Res<Time>,
    mut data: ResMut<SoundData>,
    mut teacher_tired_events: EventReader<TeacherTiredEvent>,
) {
    for e in teacher_tired_events.read() {
        let now = time.elapsed_seconds_f64();
        data.teacher_tired
            .update(now, &e.teacher, e.short_action, e.long_action)
    }
}

pub struct SoundViewPlugin;

impl Plugin for SoundViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundData::default())
            .add_systems(Startup, load_resources)
            .add_systems(PreUpdate, listen_events_teacher_tired)
            .add_systems(Update, play_track)
            .add_systems(PreUpdate, listen_reactions)
            .add_systems(PreUpdate, listen_monster_attack);
    }
}

#[derive(Resource, Default)]
struct SoundData {
    ///(audio,duration,started)
    tracks: Vec<(Handle<AudioSource>, f64, f64)>,
    tracks_last_used_index: i8,
    monsters: Vec<Handle<AudioSource>>,
    monsters_last_used_index: i8,
    reactions: HashMap<Reaction, Vec<(Handle<AudioSource>, f64)>>,

    teacher_tired: TeacherTired,
}
