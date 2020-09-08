use bracket_terminal::prelude::RGB;
use specs::{prelude::*, Component, VecStorage, System, ReadStorage, WriteStorage};
use std::cmp::{min, max};

pub enum Direction {
    North, South, East, West
}

// MAP
// TODO: Organize map implementation
#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Empty, Wall
}

fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * 80) + x as usize
}

pub fn generate_tile_map(width: usize, height: usize) -> Vec<Tile> {
    let mut map = vec![Tile::Empty; width * height];

    // Make map edges walls
    for x in 0..width {
        map[xy_idx(x, 0)] = Tile::Wall;
        map[xy_idx(x, height - 1)] = Tile::Wall;
    }

    for y in 0..height {
        map[xy_idx(0, y)] = Tile::Wall;
        map[xy_idx(width - 1, y)] = Tile::Wall;
    }

    map
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
    let map = world.fetch::<Vec<Tile>>();

    let (delta_x, delta_y) = match dir {
        Direction::North => { (0, -1) },
        Direction::South => { (0, 1) }
        Direction::East => { (-1, 0) }
        Direction::West => { (1, 0) }
    };

    for (_player, pos) in (&mut player, &mut positions).join() {
        let destination_idx = xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);
        if map[destination_idx] != Tile::Wall {
        // TODO: Implement bounding function in utils.rs
            pos.x = max(0, min(79, pos.x + delta_x));
            pos.y = max(0, min(49, pos.y + delta_y));
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
