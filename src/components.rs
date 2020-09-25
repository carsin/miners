use bracket_terminal::prelude::RGB;
use specs::{Component, VecStorage};

#[derive(PartialEq, Copy, Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: char,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {}

// generates lists of tiles visible from position and their light levels.
// I need to seperate these functionalities somehow
// FOV & light?
//
// if I do seperate them, it means the shadowcast function will run twice
// effectively making it twice as slow in an avoidable scenario
// how can I keep these from running twice?
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>, // positions relative to algorithm that are visible
    // pub light_levels: Vec<Option<f32>>, // light levels for visible tiles
    pub emits_light: bool,
    pub strength: f32, // affects how far light extends from source & light level dropoff rate from origin
    pub dirty: bool, // has game changed (player moved)?
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Monster {}
