use bevy::{prelude::*};
use itertools::Itertools;
use std::{env, cmp::Ordering};

mod colors;
use colors::*;
mod ui;
use ui::*;

// will be used for a tile background size as well.
const TILE_PLACEHOLDER_SIZE: f32 = 64.0;
// tile_size is the size of the tile that will sit on top
// of the tile placeholder. It's slightly bigger to enable
// border color effect when layering the tile on top of the placeholder.
const TILE_SIZE: f32 = 60.0;
const COLUMN_SPACER: f32 = 5.0;
const COLUMN_PADDING: f32 = 20.0;
const ROW_SPACER: f32 = 6.0;
const ROW_PADDING: f32 = 20.0;
// max of 5 guesses. range 0.=5
const LAST_GUESS_INDEX: u8 = 5;

#[derive(Debug, Component)]
struct Board {
    columns: u8,
    rows: u8,
    height: f32,
    width: f32,
}

impl Board {
    fn new(columns: u8, rows:u8) -> Self {
        // calculating width and height using
        // tile_background_size because that's the size of the whole tile.
        // the tile will sit on top of the background.
        //
        // get_spacers takes into account the fact it only needs
        // space in between columns/rows. Should be no trailing or leading spacers
        //
        // multiplying the padding * 2 because we want padding top/bottom/left/right
        let width = f32::from(columns)
            * TILE_PLACEHOLDER_SIZE
            + Board::get_spacers(columns) * COLUMN_SPACER
            + COLUMN_PADDING * 2.0;
        let height = f32::from(rows)
            * TILE_PLACEHOLDER_SIZE
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
            -self.width / 2.0 + 0.5 * TILE_PLACEHOLDER_SIZE
            + COLUMN_PADDING;
        offset
            + f32::from(col) * TILE_PLACEHOLDER_SIZE
            + f32::from(col) * COLUMN_SPACER
    }
    fn row_position_to_physical(&self, row: u8) -> f32 {
        let offset =
            -self.height / 2.0 + 0.5 * TILE_PLACEHOLDER_SIZE
            + ROW_PADDING;
        offset
            + f32::from(row) * TILE_PLACEHOLDER_SIZE
            + f32::from(row) * ROW_SPACER
    }
    /// val - number of rows/columns
    /// returns how many spacers are required based off how many rows/columns
    /// for the board.
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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
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

/// Updates the current guess being entered by player.
/// The user input was already handled, this event takes that
/// and updates the board.
#[derive(Debug)]
struct GuessUpdateEvent {
    action: GuessUpdateAction,
    key: String,
}

#[derive(Debug)]
enum GuessUpdateAction {
    Delete, // delete last character from guess
    Append, // append submitted key to guess
    Submit, // submit guess
}

impl std::fmt::Display for GuessUpdateAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessUpdateAction::Delete => write!(f, "{}", "Delete"),
            GuessUpdateAction::Append => write!(f, "{}", "Append"),
            GuessUpdateAction::Submit => write!(f, "{}","Submit"),
        }
    }
}

impl TryFrom<String> for GuessUpdateAction {
    type Error = &'static str;
    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        match value.as_str() {
            BACK_KEY => Ok(GuessUpdateAction::Delete),
            ENTER_KEY => Ok(GuessUpdateAction::Submit),
            // input comes from keyboard painted on screen.
            // only valid input can come from there so if its
            // not back key or enter key, it's gotta be a valid
            // character that should be appended to the guess if possible
            _ => Ok(GuessUpdateAction::Append),
        }
    }
}

#[derive(Default)]
struct GameContext {
    // A game of wordle can take up to 5 guesses.
    // 1 guess is being maintained at a time.
    guess_collection: Vec<String>,
    score: u32,
}

impl GameContext {
    /// returns the index to guess_collection for the last guess.
    fn get_guess_index(&self) -> usize {
        self.guess_collection.len() - 1
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum RunState {
    Playing,
    GameOver
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameUiPlugin)
        .init_resource::<FontSpec>()
        .init_resource::<GameContext>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_event::<GuessUpdateEvent>()
        .add_state(RunState::Playing)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
                .with_system(guess_update_handler)
        )
        .add_system_set(
            SystemSet::on_enter(RunState::Playing)
                .with_system(game_reset)
                .with_system(spawn_tiles)
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// spawn a wordle game board.
fn spawn_board(mut commands: Commands) {
    // 5 columns (the word to guess is a 5 letter word)
    // 6 rows (user gets 6 guesses)
    let board = Board::new(5, 6);
    // spawn wordle board
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
            // move the board up 100 units
            transform: Transform::from_xyz(0.0, 100.0, 1.0),
            ..Default::default()
    })
    .with_children(|builder| {
        // tile placeholders
        for tile in (0..board.columns)
            .cartesian_product(0..board.rows) {
                // spawn tile placeholder.
                builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: MATERIALS.tile_placeholder,
                        custom_size: Some(Vec2::new(
                            TILE_PLACEHOLDER_SIZE, TILE_PLACEHOLDER_SIZE,
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

fn spawn_tile(
    commands: &mut Commands,
    board: &Board,
    font_spec: &Res<FontSpec>,
    pos: Position,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.tile,
                custom_size: Some(Vec2::new(
                    TILE_SIZE, TILE_SIZE,
                )),
                ..Sprite::default()
            },
            transform: Transform::from_xyz(
                board.column_position_to_physical(
                    pos.x
                ),
                board.row_position_to_physical(
                    pos.y
                ) + 100.0, // matching the board's offset that makes room for keyboard
                3.0,
            ),
            ..Default::default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font_spec
                                .family
                                .clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        TextAlignment {
                            vertical:
                                VerticalAlign::Center,
                            horizontal:
                                HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform::from_xyz(
                        0.0, 0.0, 1.0
                    ),
                    ..Default::default()
                })
                .insert(TileText);
        })
        .insert(Position{x: pos.x, y: pos.y});
}

fn spawn_tiles(
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();
    for (x, y) in (0..board.columns)
        .cartesian_product(0..board.rows) {
        spawn_tile(&mut commands, board, &font_spec, Position{x,y});
    }
}
fn guess_update_handler(
    mut guess_reader: EventReader<GuessUpdateEvent>,
    mut commands: Commands,
    // p0 - tile texts from board.
    // p1 - MessageText for displaying messages to user
    mut text_set: ParamSet<(Query<&mut Text, With<TileText>>,
                            Query<&mut Text, With<MessageText>>)>,
    mut tiles: Query<
        (&Position, &Children),
        >,
    font_spec: Res<FontSpec>,
    mut game_context: ResMut<GameContext>,
) {
    let guess_index = game_context.get_guess_index();
    // row 0 of the board is at the bottom..
    // I want the guesses to display from top to bottom not bottom to top.
    // reversing the display starting position is needed.
    // LAST_GUESS_INDEX - index of guess will give me the correct Y position.
    // EX: last guess index (5) - guess index (0) = 5.
    // guess_index increments to 1 after user submits guess then 5 - 1 = 4.
    let guess_display_index = LAST_GUESS_INDEX as usize - guess_index;
    let guess: &mut String = &mut game_context.guess_collection[guess_index];
    // update the guess..
    for event in guess_reader.iter() {
        // update guess or submit
        match event.action {
            GuessUpdateAction::Delete => {guess.pop();}
            GuessUpdateAction::Append => {guess.push_str(event.key.as_str());},
            GuessUpdateAction::Submit => {
                let mut message_display_text = text_set.p1();
                if guess.len() < 5 {
                    let mut msg_text = message_display_text
                                    .get_single_mut()
                                    .expect("expect message text to exist.");
                    let mut msg_section = msg_text.sections
                                        .first_mut()
                                        .expect("expect first text section to be accessible as mutable");
                    msg_section.value = "5 characters required to submit guess.".to_string();
                }
            },
        }
        //
        match event.action {
            GuessUpdateAction::Delete |
            GuessUpdateAction::Append => {
                // update board now with this guess information.
                let mut it = tiles
                    .iter_mut()
                    .filter(|(pos, _children)|{
                        // only want tiles that are in the same rows as the
                        // guess we are working with
                        pos.y as usize == guess_display_index
                    })
                    .sorted_by(|a, b|{
                        // order by column
                        match Ord::cmp(&a.0.x, &b.0.x) {
                            a => a
                        }
                    });
                let mut guess_chars = guess.chars();
                // while there are still tiles to process
                while let Some((position, children)) = it.next() {
                    if let Some(entity) = children.first() {
                        if position.x as usize > guess.len() {
                            break
                        }
                        let mut tile_texts = text_set.p0();
                        let mut text = tile_texts
                            .get_mut(*entity)
                            .expect("expected Text to exist");
                        let mut text_section = text.sections.first_mut()
                            .expect("expect first section to be accessible as mutable");
                        // get the next character of the guess
                        match guess_chars.next() {
                            // got a character. put that in the tile
                            Some(c) => text_section.value = c.to_string(),
                            // no character there clear out the tile.
                            None => text_section.value = "".to_string(),
                        }
                    }
                }
            }
            _ => ()
        }
    }
}


fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<GameContext>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.guess_collection = vec!["".to_string()];
}