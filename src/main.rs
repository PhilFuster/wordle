use bevy::prelude::*;
use itertools::Itertools;
use std::{env};

mod colors;
use colors::*;

const TILE_SIZE: f32 = 60.0;
const TILE_BACKGROUND_SIZE: f32 = 62.0;
const COLUMN_SPACER: f32 = 5.0;
const ROW_SPACER: f32 = 6.0;
const PADDING: f32 = 20.0;

#[derive(Debug, Component)]
struct Board {
    columns: u8,
    rows: u8,
    height: f32,
    width: f32,
}

impl Board {
    fn new(columns: u8, rows:u8) -> Self {
        let width = f32::from(columns)
            * TILE_BACKGROUND_SIZE
            + f32::from(columns + 1) * COLUMN_SPACER;
        let height = f32::from(rows)
            * TILE_BACKGROUND_SIZE
            + f32::from(rows + 1) * ROW_SPACER;
        Board {
            columns,
            rows,
            height,
            width,
        }
    }

    fn column_position_to_physical(&self, col: u8) -> f32 {
        let offset =
            -self.width/ 2.0 + 0.5 * TILE_BACKGROUND_SIZE;
        offset
            + f32::from(col) * TILE_BACKGROUND_SIZE
            + f32::from(col + 1) * COLUMN_SPACER
    }
    fn row_position_to_physical(&self, row: u8) -> f32 {
        let offset =
            -self.height/ 2.0 + 0.5 * TILE_BACKGROUND_SIZE;
        offset
            + f32::from(row) * TILE_BACKGROUND_SIZE
            + f32::from(row + 1) * COLUMN_SPACER
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
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_game_board)
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_game_board(mut commands: Commands) {
    let board = Board::new(5, 6);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    board.width,
                    board.height,
                )),
                ..Sprite::default()
            },
            ..Default::default()
    })
    .with_children(|builder| {
        for tile in (0..board.columns)
            .cartesian_product(0..board.rows) {
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

fn spawn_tiles(
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();
    

}