use bevy::prelude::Color;

pub struct Materials {
    pub board: Color,
    pub tile_placeholder: Color,
    pub tile: Color,
    pub none: Color,
}

pub const MATERIALS: Materials = Materials {
    board: Color::rgb(1.0, 1.0, 1.0),
    tile_placeholder: Color::rgb(0.5, 0.5, 0.5),
    tile: Color::rgb(1.0, 1.0, 1.0),
    none: Color::NONE,
};

pub struct KeyboardMaterials {
    pub kb_btn_background: Color,
    pub kb_btn_letter: Color,
    // letter is in word, but wrong spot.
    pub kb_btn_background_wrong_spot: Color,
    // letter is in word, right spot.
    pub kb_btn_background_right_spot: Color,
    // letter is not in word.
    pub kb_btn_background_not_in_word: Color,
}

pub const KEYBOARD_MATERIALS: KeyboardMaterials =
    KeyboardMaterials {
        kb_btn_background: Color::rgb(0.9, 0.9, 0.9),
        kb_btn_letter: Color::BLACK,
        kb_btn_background_wrong_spot: Color::rgb(1.0,1.0,0.0),
        kb_btn_background_right_spot: Color::rgb(0.0,1.0,0.0),
        kb_btn_background_not_in_word: Color::rgb(0.9,0.9,0.9),
    };