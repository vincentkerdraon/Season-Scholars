use bevy::prelude::*;

use crate::model::overlord::*;

const INSTRUCTION: &str = "press \"reset\" to reach menu";
const TITLE: &str = "Season Scholars";

fn load_resources(mut commands: Commands, mut data: ResMut<RecapData>) {
    data.background = commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Px(0.),
                bottom: Val::Px(0.),
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgba(0.85, 0.7, 0.7, 0.85)),
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(300),
            ..Default::default()
        })
        .id();

    data.title = commands
        .spawn(TextBundle {
            text: Text::from_section(
                TITLE,
                TextStyle {
                    font_size: 60.0,
                    color: Color::rgb(0.3, 0.3, 0.2),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Relative,
                align_content: AlignContent::Center,
                left: Val::Px(20.),
                bottom: Val::Px(-20.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(304),
            ..default()
        })
        .id();

    data.recap = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font_size: 50.0,
                    color: Color::rgb(0.2, 0.2, 0.2),

                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Relative,
                align_content: AlignContent::FlexStart,
                left: Val::Px(500.),
                bottom: Val::Px(-400.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(304),
            ..default()
        })
        .id();

    data.instructions = commands
        .spawn(TextBundle {
            text: Text::from_section(
                INSTRUCTION,
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.2, 0.2, 0.2),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                align_content: AlignContent::Start,
                left: Val::Px(750.),
                bottom: Val::Px(20.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(304),
            ..default()
        })
        .id();
}

fn listen_events(
    mut data: ResMut<RecapData>,
    mut display_screen_menu_events: EventReader<DisplayScreenMenuEvent>,
    mut display_screen_game_events: EventReader<DisplayScreenGameEvent>,
    mut display_screen_game_over_recap_events: EventReader<DisplayScreenGameOverRecapEvent>,
    mut param_set: ParamSet<(Query<(&mut Text, &mut Visibility)>, Query<&mut Visibility>)>,
) {
    let mut dirty = false;
    for _ in display_screen_game_events.read() {
        if data.display {
            data.display = false;
            dirty = true;
        }
    }

    for _ in display_screen_menu_events.read() {
        if data.display {
            data.display = false;
            dirty = true;
        }
    }

    for e in display_screen_game_over_recap_events.read() {
        data.display = true;
        dirty = true;
        data.event_data = e.clone();
    }

    if !dirty {
        return;
    }

    if !data.display {
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.title) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.recap) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.instructions) {
            *visibility = Visibility::Hidden;
        }
        if let Ok(mut visibility) = param_set.p1().get_mut(data.background) {
            *visibility = Visibility::Hidden;
        }

        return;
    }

    if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.title) {
        *visibility = Visibility::Visible;
    }
    if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.instructions) {
        *visibility = Visibility::Visible;
    }
    if let Ok((mut text, mut visibility)) = param_set.p0().get_mut(data.recap) {
        text.sections[0].value = format!(
            "- GAME OVER -\nReason: \t{}\nScore: \t{}\nPlayers: \t{}\nDuration: \t{} seconds, {} seasons",
            data.event_data.reason,
            data.event_data.score,
            data.event_data.teachers.len(),
            data.event_data.time_since_start_s.ceil(),
            data.event_data.seasons_elapsed
        );
        *visibility = Visibility::Visible;
    }
    if let Ok(mut visibility) = param_set.p1().get_mut(data.background) {
        *visibility = Visibility::Visible;
    }
}

pub struct RecapViewPlugin;

impl Plugin for RecapViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RecapData::new())
            .add_systems(Startup, load_resources)
            .add_systems(PreUpdate, listen_events);
    }
}

#[derive(Resource)]
struct RecapData {
    title: Entity,
    recap: Entity,
    instructions: Entity,
    background: Entity,

    display: bool,
    event_data: DisplayScreenGameOverRecapEvent,
}

impl RecapData {
    pub fn new() -> Self {
        Self {
            title: Entity::PLACEHOLDER,
            recap: Entity::PLACEHOLDER,
            instructions: Entity::PLACEHOLDER,
            background: Entity::PLACEHOLDER,
            display: false,
            event_data: DisplayScreenGameOverRecapEvent { ..default() },
        }
    }
}
