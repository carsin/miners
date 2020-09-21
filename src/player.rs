use specs::{WorldExt, World, prelude::*};
use super::{Direction, Player, Position, map, util};

pub fn move_player(dir: Direction, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut player = world.write_storage::<Player>();
    let map = world.fetch::<map::Map>();

    let (delta_x, delta_y) = match dir {
        Direction::North => { (0, -1) },
        Direction::South => { (0, 1) }
        Direction::East => { (-1, 0) }
        Direction::West => { (1, 0) }
    };

    for (_player, pos) in (&mut player, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != map::TileType::Wall {
            pos.x = util::clamp(pos.x + delta_x, 0, 79);
            pos.y = util::clamp(pos.y + delta_y, 0, 49);
        }
    }
}
