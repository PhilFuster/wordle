use bevy::{prelude::*};
use itertools::Itertools;
use std::{env, cmp::Ordering};

mod colors;
use colors::*;
mod ui;
use ui::*;

// tile_background_size is slightly bigger than tile_size which allows for a
// border to show
const TILE_BACKGROUND_SIZE: f32 = 64.0;
// tile_size is the size of the actual tile that will sit on top
// of the tile background.
const TILE_SIZE: f32 = 60.0;
const COLUMN_SPACER: f32 = 5.0;
const COLUMN_PADDING: f32 = 20.0;
const ROW_SPACER: f32 = 6.0;
const ROW_PADDING: f32 = 20.0;

#[derive(Debug, Component)]
struct Board {
    columns: u8,
    rows: u8,
    height: f32,
    width: f32,
}

impl Board {
    fn new(columns: u8, rows:u8) -> Self {
        // when calculating width and height using
        // tile_background_size because that's the size of the whole tile.
        // the tile will sit right on top of the background.
        //
        // get_spacers takes into account the fact it only needs
        // space in between columns/rows. Should be no trailing spacers
        //
        // multiplying the padding * 2 because we want padding top/bottom/left/right
        let width = f32::from(columns)
            * TILE_BACKGROUND_SIZE
            + Board::get_spacers(columns) * COLUMN_SPACER
            + COLUMN_PADDING * 2.0;
        let height = f32::from(rows)
            * TILE_BACKGROUND_SIZE
            + Board::get_spacers(rows) * ROW_SPACER
            + ROW_PADDING * 2.0;
        Board {
            columns,
            rows,
            height,
            width,
        }
    }

    fn column_position_to_physical(&self, col: u8) -> f32 {
        // columns go from left to right.
        // take negative width of the board and divide by 2.
        // multiple by half of the background size and add the
        // column padding.
        let offset =
            -self.width / 2.0 + 0.5 * TILE_BACKGROUND_SIZE
            + COLUMN_PADDING;
        offset
            + f32::from(col) * TILE_BACKGROUND_SIZE
            + f32::from(col) * COLUMN_SPACER
    }
    fn row_position_to_physical(&self, row: u8) -> f32 {
        let offset =
            -self.height / 2.0 + 0.5 * TILE_BACKGROUND_SIZE
            + ROW_PADDING;
        offset
            + f32::from(row) * TILE_BACKGROUND_SIZE
            + f32::from(row) * ROW_SPACER
    }

    fn get_spacers(val: u8) -> f32 {
        match Ord::cmp(&val, &1){
            Ordering::Less => f32::from(val),
            Ordering::Equal
                | Ordering::Greater => f32::from(val - 1),
        }
    }
}

#[derive(
    Debug, Eq, PartialEq, Hash, Copy, Clone, Component
)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;

struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        let dir = env::current_dir().unwrap();
        let full_path = dir.join("assets/fonts/FiraCode-Bold.ttf");
        FontSpec {
            family: asset_server
                .load(full_path)
        }
    }
}

#[derive(Default)]
struct Game {
    guesses: Vec<String>
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameUiPlugin)
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_game_board)
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// spawn a wordle game board.
/// 5 columns (because the word to guess is a 5 letter word)
/// 6 rows (because the user gets 6 guesses)
fn spawn_game_board(mut commands: Commands) {
    let board = Board::new(5, 6);
    // spawn game board
    commands
        // board background
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    board.width,
                    board.height,
                )),
                ..Sprite::default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 1.0),
            ..Default::default()
    })
    .with_children(|builder| {
        // creating tiles
        for tile in (0..board.columns)
            .cartesian_product(0..board.rows) {
                // spawn tile background.
                // it's slightly bigger than the tile
                // created next, right on top, in order
                // to create a border effect.
                builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: MATERIALS.tile_placeholder,
                        custom_size: Some(Vec2::new(
                            TILE_BACKGROUND_SIZE, TILE_BACKGROUND_SIZE,
                        )),
                        ..Sprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.column_position_to_physical(
                            tile.0
                        ),
                        board.row_position_to_physical(
                            tile.1
                        ),
                        1.0,
                    ),
                    ..Default::default()
                });
                // spawn tile
                builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: MATERIALS.tile,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..Sprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.column_position_to_physical(
                            tile.0
                        ),
                        board.row_position_to_physical(
                            tile.1
                        ),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
    })
    .insert(board);
}