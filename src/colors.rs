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
}

pub const KEYBOARD_MATERIALS: KeyboardMaterials =
    KeyboardMaterials {
        kb_btn_background: Color::rgb(0.9, 0.9, 0.9),
        kb_btn_letter: Color::BLACK,
    };