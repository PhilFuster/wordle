use crate::colors::{KEYBOARD_MATERIALS,KeyboardMaterials, MATERIALS};
use crate::{FontSpec, GameContext, Board, Position, GuessUpdateAction, GuessUpdateEvent};
use bevy::prelude::*;

const KEYBOARD_LETTERS: [&str; 28] = [
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "A", "S", "D", "F", "G", "H", "J", "K", "L",
    "ENTER", "Z", "X", "C", "V", "B", "N", "M", "<-"
    ];

pub const ENTER_KEY: &str = "ENTER";
pub const BACK_KEY: &str = "<-";
pub struct GameUiPlugin;
#[derive(Component)]
pub struct MessageText;


impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App,) {
        app.add_startup_system(setup_ui)
            .add_system(keyboard_button_interaction_system);
    }
}

fn setup_ui(
    mut commands: Commands,
    font_spec: Res<FontSpec>,
) {
    // spawn the camera so people can see it lol
    commands.spawn_bundle(UiCameraBundle::default());
    // ui container - will hold the menu at the top and the keyboard at the bottom
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0
        ),
        color: UiColor(Color::NONE),
        ..Default::default()
    }).with_children(|ui_container| {
        // spawn menu
        ui_container.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: UiColor(MATERIALS.none),
            ..Default::default()
        })
        .with_children(|parent| {
            // game title
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Auto),
                    justify_content: JustifyContent::FlexStart,
                    padding: Rect {
                        left: Val::Px(30.0),
                        right: Val::Px(30.0),
                        top: Val::Px(30.0),
                        bottom: Val::Px(30.0),
                    },
                    ..Default::default()
                },
                color: UiColor(MATERIALS.none),
                ..Default::default()
            }).with_children(|builder| {
                builder.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Wordle",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
            // message display container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Auto),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                color: UiColor(MATERIALS.none),
                ..Default::default()
            }).with_children(|builder| {
                builder.spawn_bundle(NodeBundle {
                    // message display bundle
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: Rect {
                            left: Val::Percent(20.0),
                            right: Val::Percent(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0)
                        },
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    color: UiColor(MATERIALS.none),
                    ..Default::default()
                }).with_children(|builder| {
                    builder.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Message Board",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::WHITE
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                    builder.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "{Default Text}",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::WHITE
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(MessageText);
                });
            });
        });
        // spawn keyboard
        ui_container.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0
        ),
        color: UiColor(Color::WHITE),
        ..Default::default()
    })
    .with_children(|kb_builder| {
        // keyboard tiles
        // row 1
        kb_builder.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Px(40.0)),
                    align_items: AlignItems::FlexStart,
                    margin: Rect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(30.0),
                    },
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::WHITE),
                ..Default::default()
            })
            .with_children(|builder| {
                for key_index in 0..=9 {
                    // call spawn_keyboard_key
                    spawn_keyboard_button(builder, &font_spec, key_index);
                }
            });
            // row 2
            kb_builder.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Px(40.0)),
                    align_items: AlignItems::FlexStart,
                    margin: Rect {
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(30.0),
                    },
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::WHITE),
                ..Default::default()
            })
            .with_children(|builder| {
                for key_index in 10..=18 {
                    // call spawn_keyboard_key
                    spawn_keyboard_button(builder, &font_spec, key_index);
                }
            });
            // row 3
            kb_builder.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Px(40.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: UiColor(Color::WHITE),
                ..Default::default()
            })
            .with_children(|builder| {
                for key_index in 19..=27 {
                    // call spawn_keyboard_key
                    spawn_keyboard_button(builder, &font_spec, key_index);
                }
            });
        });
    });
}


fn spawn_keyboard_button(
    commands: &mut ChildBuilder,
    font_spec: &Res<FontSpec>,
    pos: usize,
) {
    commands
        .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: Rect {
                        left: Val::Px(0.0),
                        right: Val::Px(10.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    ..Default::default()
                },
                color: UiColor(KEYBOARD_MATERIALS.kb_btn_background),
                ..Default::default()
            })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        KEYBOARD_LETTERS[pos],
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 20.0,
                            color: KEYBOARD_MATERIALS.kb_btn_letter
                        },
                        Default::default()
                    ),
                    ..Default::default()
                });
            });
}

fn keyboard_button_interaction_system(
    interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>)
    >,
    text_query: Query<&Text>,
    mut guess_writer: EventWriter<GuessUpdateEvent>,
    game_context: Res<GameContext>,
) {
    for (interaction, children) in
    interaction_query.iter() {
        match interaction {
            // only handling clicked events here..
            Interaction::Clicked => {
                    let guess_index = game_context.get_guess_index();
                    let guess = &game_context.guess_collection[guess_index];
                    // keyboard_button entity implemented such that the 1st child
                    // is the TextBundle.
                    let text = text_query.get(*children.first().expect(
                        "expect button have a first child."
                    ))
                    .unwrap();
                    // determine the kind of GuessUpdateAction taking place
                    // based on the key that was pressed.
                    let text_section = text.sections.first()
                        .expect("Expect first section to be accessible as reference");
                    let key = text_section.value.to_string();
                    // turn key pressed in to a GuessUpdateAction
                    let update_action = GuessUpdateAction::try_from(text_section.value.to_string()).ok();
                    if let Some(action) = update_action {
                        // validate whether a guess update can happen
                        // based on the action.
                        match action {
                            GuessUpdateAction::Append => {
                                if guess.len() > 4 {
                                    // max len is 5.
                                    // greater than 4 no good
                                    continue;
                                }
                                guess_writer.send(GuessUpdateEvent{action: GuessUpdateAction::Append, key})
                            }
                            GuessUpdateAction::Delete => {
                                if guess.len() < 1 {
                                    // nothing to delete for the guess
                                    continue;
                                }
                                guess_writer.send(GuessUpdateEvent{action: GuessUpdateAction::Delete, key})
                            }
                            GuessUpdateAction::Submit => {
                                guess_writer.send(GuessUpdateEvent{action: GuessUpdateAction::Submit, key})
                            }
                        }
                    }
                }
                _ => ()
            }
    }
}