use bracket_terminal::prelude::RGB;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Renderable {
    glyph: char,
    fg: RGB,
    bg: RGB,
}
