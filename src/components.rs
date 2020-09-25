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
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>, // positions relative to algorithm that are visible
    pub emitter: Option<f32>, // determines if entity emits light, and if so the maximum strength of 1.0 to 0.0
    pub range: f32, // changes how deep the shadowcasting algorithm goes. affects fov viewrange & lightshed
    // pub max_strength: f32, // changes the light level at the source and thus how gradual the light shift is
    pub dirty: bool, // has game changed (player moved)?
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Monster {}
