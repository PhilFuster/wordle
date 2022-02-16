use crate::colors::{KEYBOARD_MATERIALS, KeyboardMaterials, MATERIALS};
use crate::{FontSpec, Game, Board, Position};
use bevy::prelude::*;

const KEYBOARD_LETTERS: [&str; 28] = [
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "A", "S", "D", "F", "G", "H", "J", "K", "L",
    "ENTER", "Z", "X", "C", "V", "B", "N", "M", "<-"
    ];
pub struct GameUiPlugin;


impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App,) {
        app.add_startup_system(setup_ui);
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
