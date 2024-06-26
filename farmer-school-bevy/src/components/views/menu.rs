use std::collections::HashSet;

use bevy::prelude::*;

use crate::model::config::*;

use crate::model::overlord::*;

const EXPLAIN: &str = "You play as a wise teacher guiding farmers through the changing seasons. \n
Each season brings new crops to learn: asparagus, cherries, chanterelles and lemons. \n
But beware! Monsters with unique needs threaten your village. \n
To protect your people, you must graduate students with the precise knowledge to combat these threats. \n
Will you rise to the challenge?";
const HOW_TO: &str = "- Move your teacher from station to station.\n
- In front of the students, you can teach them about the current season (short action) \nor graduate them to fullfil the current monster needs.\n
- At the door in the back, you can welcome a new student (short action) or find a new one (long action).\n
- Through the magical windows on the left, you can spy on incoming monsters to learn their needs (short action) \nor repair the portal when the classroom get damaged (long action).\n
- At the cooking station, you can eat to rest and move faster (short action) or cook (long action). Students eat every winter.\n";

const JOIN: &str = "press \"short action\" to join/leave\n
press \"long action\" to start the game\n
press \"reset\" to exit";
const TITLE: &str = "Season Scholars";

fn credits() -> String {
    format!(
        "Created for the 2024 Calgary game jam \"arcade\"\n\
        Maelle & Vincent KERDRAON\n\
        This version {} using rust + bevy, another version in godot.\n\
        Only works in 1920x1080.\n\
        All images using AGI. Homemade sounds.\n\
    {}",
        VERSION, REPO
    )
}

fn load_resources(
    config: Res<Config>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut data: ResMut<MenuData>,
) {
    // WTF, IMPOSSIBLE to use the Z index?
    // replacing with image.
    // data.background = commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             left: Val::Px(0.),
    //             bottom: Val::Px(0.),
    //             width: Val::Vw(100.),
    //             height: Val::Vh(100.),
    //             ..Default::default()
    //         },
    //         background_color: BackgroundColor(Color::rgba(0.8, 0.8, 0.8, 0.85)),
    //         visibility: Visibility::Hidden,
    //         z_index: ZIndex::Global(300),
    //         ..Default::default()
    //     })
    //     .id();

    data.images.push(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(
                    config
                        .base_path
                        .join("images/ready/StartScreen/Gradient.png"),
                ),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 300.,
                    },
                    scale: Vec3 {
                        x: 10.,
                        y: 6.,
                        z: 1.,
                    },
                    ..default()
                },

                ..default()
            })
            .id(),
    );

    data.images.push(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(
                    config
                        .base_path
                        .join("images/ready/StartScreen/TeacherA.png"),
                ),
                transform: Transform {
                    translation: Vec3 {
                        x: -700.,
                        y: -100.,
                        z: 301.,
                    },
                    scale: Vec3 {
                        x: 0.8,
                        y: 0.8,
                        z: 1.,
                    },
                    ..default()
                },

                ..default()
            })
            .id(),
    );

    data.images.push(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(
                    config
                        .base_path
                        .join("images/ready/StartScreen/TeacherB.png"),
                ),
                transform: Transform {
                    translation: Vec3 {
                        x: 700.,
                        y: -100.,
                        z: 301.,
                    },
                    scale: Vec3 {
                        x: 0.8,
                        y: 0.8,
                        z: 1.,
                    },
                    ..default()
                },

                ..default()
            })
            .id(),
    );

    data.input_arcade = commands
        .spawn(SpriteBundle {
            texture: asset_server.load(
                config
                    .base_path
                    .join("images/ready/StartScreen/InputArcade.png"),
            ),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -100.,
                    z: 301.,
                },
                scale: Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 1.,
                },
                ..default()
            },

            ..default()
        })
        .id();

    data.input_keyboard = commands
        .spawn(SpriteBundle {
            texture: asset_server.load(
                config
                    .base_path
                    .join("images/ready/StartScreen/InputKeyboard1.png"),
            ),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -100.,
                    z: 302.,
                },
                scale: Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 1.,
                },
                ..default()
            },
            ..default()
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

    data.explain = commands
        .spawn(TextBundle {
            text: Text::from_section(
                EXPLAIN,
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.2, 0.2, 0.2),

                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                align_content: AlignContent::FlexStart,
                left: Val::Px(500.),
                bottom: Val::Px(700.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(304),
            ..default()
        })
        .id();

    data.join_instructions = commands
        .spawn(TextBundle {
            text: Text::from_section(
                JOIN,
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

    data.player_a = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "PlayerA",
                TextStyle {
                    font_size: 50.0,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                align_content: AlignContent::Center,
                left: Val::Px(100.),
                bottom: Val::Px(20.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(310),
            ..default()
        })
        .id();

    data.player_b = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "PlayerB",
                TextStyle {
                    font_size: 50.0,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                align_content: AlignContent::Center,
                left: Val::Px(1400.),
                bottom: Val::Px(20.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(310),
            ..default()
        })
        .id();
}

fn listen_events(
    time: Res<Time>,
    mut data: ResMut<MenuData>,
    mut display_screen_menu_events: EventReader<DisplayScreenMenuEvent>,
    mut display_screen_game_events: EventReader<DisplayScreenGameEvent>,
    mut display_screen_game_over_recap_events: EventReader<DisplayScreenGameOverRecapEvent>,
    mut param_set: ParamSet<(Query<(&mut Text, &mut Visibility)>, Query<&mut Visibility>)>,
) {
    let mut dirty = false;
    for _ in display_screen_game_events.read() {
        data.display = false;
        dirty = true;
    }
    for _ in display_screen_game_over_recap_events.read() {
        data.display = false;
        dirty = true;
    }
    for e in display_screen_menu_events.read() {
        data.display = true;
        data.teachers = HashSet::new();
        for t in e.teachers.clone() {
            data.teachers.insert(t);
        }
        dirty = true;
    }

    if data.display {
        let now = time.elapsed_seconds_f64();
        if data.next_switch_variation < now {
            data.next_switch_variation = now + 6.;
            data.variation += 1;
            dirty = true;
            trace!("menu switch");
        }
    }

    if !dirty {
        return;
    }

    if !data.display {
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.player_a) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.player_b) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.title) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.explain) {
            *visibility = Visibility::Hidden;
        }
        if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.join_instructions) {
            *visibility = Visibility::Hidden;
        }
        if let Ok(mut visibility) = param_set.p1().get_mut(data.input_arcade) {
            *visibility = Visibility::Hidden;
        }
        if let Ok(mut visibility) = param_set.p1().get_mut(data.input_keyboard) {
            *visibility = Visibility::Hidden;
        }
        for e in data.images.iter() {
            if let Ok(mut visibility) = param_set.p1().get_mut(*e) {
                *visibility = Visibility::Hidden;
            }
        }

        return;
    }

    if let Ok((mut text, mut visibility)) = param_set.p0().get_mut(data.player_a) {
        if data.teachers.contains(&Teacher::A) {
            text.sections[0].value = "PlayerA ready!".to_string();
            text.sections[0].style.color = Color::rgb(0.0, 0.7, 0.0);
        } else {
            text.sections[0].value = "PlayerA available".to_string();
            text.sections[0].style.color = Color::rgb(0.7, 0.0, 0.7);
        }
        *visibility = Visibility::Visible;
    }
    if let Ok((mut text, mut visibility)) = param_set.p0().get_mut(data.player_b) {
        if data.teachers.contains(&Teacher::B) {
            text.sections[0].value = "PlayerB ready!".to_string();
            text.sections[0].style.color = Color::rgb(0.0, 0.7, 0.0);
        } else {
            text.sections[0].value = "PlayerB available".to_string();
            text.sections[0].style.color = Color::rgb(0.7, 0.0, 0.7);
        }
        *visibility = Visibility::Visible;
    }
    if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.title) {
        *visibility = Visibility::Visible;
    }
    if let Ok((_, mut visibility)) = param_set.p0().get_mut(data.join_instructions) {
        *visibility = Visibility::Visible;
    }
    if let Ok((mut text, mut visibility)) = param_set.p0().get_mut(data.explain) {
        if data.variation % 3 == 0 {
            text.sections[0].value = EXPLAIN.to_string();
        } else if data.variation % 3 == 1 {
            text.sections[0].value = HOW_TO.to_string();
        } else {
            text.sections[0].value = credits();
        }
        *visibility = Visibility::Visible;
    }
    if let Ok(mut visibility) = param_set.p1().get_mut(data.input_arcade) {
        if data.variation % 2 == 0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    if let Ok(mut visibility) = param_set.p1().get_mut(data.input_keyboard) {
        if data.variation % 2 == 1 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for e in data.images.iter() {
        if let Ok(mut visibility) = param_set.p1().get_mut(*e) {
            *visibility = Visibility::Visible;
        }
    }
}

pub struct MenuViewPlugin;

impl Plugin for MenuViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuData::new())
            .add_systems(Startup, load_resources)
            .add_systems(Update, listen_events);
    }
}

#[derive(Resource)]
struct MenuData {
    title: Entity,
    explain: Entity,
    input_arcade: Entity,
    input_keyboard: Entity,
    join_instructions: Entity,
    player_a: Entity,
    player_b: Entity,
    images: Vec<Entity>,

    display: bool,
    variation: i8,
    next_switch_variation: f64,
    teachers: HashSet<Teacher>,
}

impl MenuData {
    pub fn new() -> Self {
        Self {
            title: Entity::PLACEHOLDER,
            explain: Entity::PLACEHOLDER,
            input_arcade: Entity::PLACEHOLDER,
            input_keyboard: Entity::PLACEHOLDER,
            join_instructions: Entity::PLACEHOLDER,
            player_a: Entity::PLACEHOLDER,
            player_b: Entity::PLACEHOLDER,
            images: Vec::new(),

            display: false,
            variation: 0,
            next_switch_variation: 0.,
            teachers: HashSet::new(),
        }
    }
}
