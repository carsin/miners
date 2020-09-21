use bracket_terminal::prelude::RGB;
use specs::{prelude::*, Component, VecStorage, System, ReadStorage, WriteStorage};

#[derive(Component, Debug)]
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

// Test system
// #[derive(Component, Debug)]
// #[storage(VecStorage)]
// pub struct Moving {}

// impl<'a> System<'a> for Moving {
//     type SystemData = (ReadStorage<'a, Moving>, WriteStorage<'a, Position>);

//     fn run(&mut self, (entity, mut position): Self::SystemData) {
//         for (_entity, position) in (&entity, &mut position).join() {
//             position.y += 1;
//             if position.y > 49 { position.y = 1; }
//         }
//     }
// }
