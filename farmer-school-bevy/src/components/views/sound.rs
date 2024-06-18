use std::collections::HashMap;
use std::vec;

use bevy::prelude::*;

use crate::model::config::Config;

use crate::model::kitchen::*;
use crate::model::overlord::*;
use crate::model::portal::*;
use crate::model::students::*;
use crate::model::welcome::*;
use bevy::audio::{AudioSource, PlaybackMode, Volume};

fn load_resources(
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut data: ResMut<SoundData>,
) {
    let track = asset_server.load(config.base_path_sound.join("track_1.ogg"));
    data.tracks.push((track.clone(), 20., 0.));
    let track = asset_server.load(config.base_path_sound.join("track_2.ogg"));
    data.tracks.push((track.clone(), 22., 0.));

    data.monsters
        .push(asset_server.load(config.base_path_sound.join("monster_1.ogg")));
    data.reactions.insert(
        Reaction::Fail,
        vec![asset_server.load(config.base_path_sound.join("fail_short_1.ogg"))],
    );
    data.reactions.insert(
        Reaction::Long,
        vec![
            asset_server.load(config.base_path_sound.join("success_long_1.ogg")),
            asset_server.load(config.base_path_sound.join("success_long_2.ogg")),
        ],
    );
    data.reactions.insert(
        Reaction::Short,
        vec![
            asset_server.load(config.base_path_sound.join("success_short_1.ogg")),
            asset_server.load(config.base_path_sound.join("success_short_2.ogg")),
        ],
    );
}

fn play_sound(commands: &mut Commands, h: Handle<AudioSource>, volume: f32) {
    commands.spawn((AudioBundle {
        source: h,
        settings: PlaybackSettings {
            mode: PlaybackMode::Once,
            paused: false,
            volume: Volume::new(volume),
            ..default()
        },
        ..default()
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
    play_sound(&mut commands, t.clone(), 0.3);
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
    data: Res<SoundData>,
) {
    let mut play_reaction = |reaction: Reaction| {
        let h = data
            .reactions
            .get(&reaction)
            .unwrap()
            .get(0)
            .unwrap()
            .clone();
        play_sound(&mut commands, h, 1.);
    };

    if invalid_action_station_events.read().last().is_some() {
        play_reaction(Reaction::Fail);
    }
    if teacher_ate_events.read().last().is_some() {
        play_reaction(Reaction::Short);
    }
    if cooked_events.read().last().is_some() {
        play_reaction(Reaction::Long);
    }
    if observe_portal_events.read().last().is_some() {
        play_reaction(Reaction::Short);
    }
    if portal_fixed_events.read().last().is_some() {
        play_reaction(Reaction::Long);
    }
    if graduated_events.read().last().is_some() {
        play_reaction(Reaction::Long);
    }
    if taught_events.read().last().is_some() {
        play_reaction(Reaction::Short);
    }
    if student_welcomed_events.read().last().is_some() {
        play_reaction(Reaction::Short);
    }
    if recruit_student_events.read().last().is_some() {
        play_reaction(Reaction::Long);
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
        play_sound(&mut commands, h, 1.);
    }
}

pub struct SoundViewPlugin;

impl Plugin for SoundViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundData::default())
            .add_systems(Update, play_track)
            .add_systems(PreUpdate, listen_reactions)
            .add_systems(PreUpdate, listen_monster_attack)
            .add_systems(Startup, load_resources);
    }
}

#[derive(Resource, Default)]
struct SoundData {
    ///(audio,duration,started)
    tracks: Vec<(Handle<AudioSource>, f64, f64)>,
    tracks_last_used_index: i8,
    monsters: Vec<Handle<AudioSource>>,
    monsters_last_used_index: i8,
    reactions: HashMap<Reaction, Vec<Handle<AudioSource>>>,
}
