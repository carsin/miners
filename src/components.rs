use bracket_terminal::prelude::RGB;
use specs::{prelude::*, Component, VecStorage, System, ReadStorage, WriteStorage};

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
    pub visible_tiles: Vec<Position>,
    pub range: usize,
    pub dirty: bool,
}
