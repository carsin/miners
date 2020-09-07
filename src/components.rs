use bracket_terminal::prelude::RGB;
use specs::{prelude::*, Component, VecStorage, System, ReadStorage, WriteStorage};
use std::cmp::{min, max};

pub enum Direction {
    North, South, East, West
}

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

pub fn move_player(dir: Direction, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut player = world.write_storage::<Player>();

    for (_player, pos) in (&mut player, &mut positions).join() {
        match dir {
            // TODO: Implement bounding function in utils.rs
            Direction::North => { pos.y = max(0, min(49, pos.y - 1)) }
            Direction::South => { pos.y = max(0, min(49, pos.y + 1)) }
            Direction::East => { pos.x = max(0, min(79, pos.x - 1)) }
            Direction::West => { pos.x = max(0, min(79, pos.x + 1)) }
        }
    }
}


// Test system
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Moving {}

impl<'a> System<'a> for Moving {
    type SystemData = (ReadStorage<'a, Moving>, WriteStorage<'a, Position>);

    fn run(&mut self, (entity, mut position): Self::SystemData) {
        for (_entity, position) in (&entity, &mut position).join() {
            position.y += 1;
            if position.y > 49 { position.y = 1; }
        }
    }
}
