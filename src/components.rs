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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub light_levels: Vec<Position>, // keep track of stuff in shadowcasting algorithm, not used in render
    pub strength: usize, // effects range of shadowcasting algorithm
    pub dirty: bool, // has game changed (player moved)?
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Monster {}
