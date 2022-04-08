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
    // create the keyboard the user will use to spell out words.

    // spawn the background for the keyboard
    commands.spawn_bundle(NodeBundle {
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
    .with_children(|builder| {
        // keyboard tiles
        // row 1
        builder.spawn_bundle(NodeBundle {
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
        builder.spawn_bundle(NodeBundle {
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
        builder.spawn_bundle(NodeBundle {
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
    // menu

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
                    let guess_index = game_context.guess_index;
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
                                if guess.len() != 5 {
                                    // guess length must be 5 to submit
                                    continue;
                                }
                                guess_writer.send(GuessUpdateEvent{action: GuessUpdateAction::Submit, key})
                            }
                        }
                    }
                }
                _ => ()
            }
    }
}