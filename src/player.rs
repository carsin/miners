use specs::{WorldExt, World, prelude::*};
use super::{Direction, Player, Position, TileType, map};
use std::cmp::{min, max};
use map::xy_idx;

pub fn move_player(dir: Direction, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut player = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    let (delta_x, delta_y) = match dir {
        Direction::North => { (0, -1) },
        Direction::South => { (0, 1) }
        Direction::East => { (-1, 0) }
        Direction::West => { (1, 0) }
    };

    for (_player, pos) in (&mut player, &mut positions).join() {
        let destination_idx = xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);
        if map[destination_idx] != TileType::Wall {
        // TODO: Implement bounding function in utils.rs
            pos.x = max(0, min(79, pos.x + delta_x));
            pos.y = max(0, min(49, pos.y + delta_y));
        }

    }
}
