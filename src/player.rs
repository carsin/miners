use specs::{WorldExt, World, prelude::*};
use super::{Direction, Player, Position, TileType, map, util};
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
            pos.x = util::clamp((pos.x + delta_x) as usize, 0, 79) as i32;
            pos.y = util::clamp((pos.y + delta_y) as usize, 0, 49) as i32;
        }
    }
}
