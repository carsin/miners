use specs::{WorldExt, World, prelude::*};
use bracket_terminal::prelude::*;
use super::{Direction, Player, Position, Map, util, Viewshed, Game, State};

pub fn move_player(dir: Direction, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut player = world.write_storage::<Player>();
    let mut viewsheds = world.write_storage::<Viewshed>();
    let map = world.fetch::<Map>();

    let (delta_x, delta_y) = match dir {
        Direction::North => { (0, -1) },
        Direction::South => { (0, 1) }
        Direction::East => { (1, 0) }
        Direction::West => { (-1, 0) }
    };

    for (_player, pos, viewshed) in (&mut player, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.tile_blocked[destination_idx] {
            pos.x = util::clamp(pos.x + delta_x, 0, (map.width - 1) as i32);
            pos.y = util::clamp(pos.y + delta_y, 0, (map.height - 1) as i32);

            // TODO: bracket Point -> custom position struct
            let mut player_pos = world.write_resource::<Position>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
            viewshed.dirty = true;
        }
    }
}

pub fn input(game: &mut Game, ctx: &mut BTerm) -> State {
    match ctx.key {
        None => { return State::Paused }
        Some(key) => match key {
            VirtualKeyCode::K | VirtualKeyCode::W => move_player(Direction::North, &mut game.world),
            VirtualKeyCode::J | VirtualKeyCode::S => move_player(Direction::South, &mut game.world),
            VirtualKeyCode::L | VirtualKeyCode::D => move_player(Direction::East, &mut game.world),
            VirtualKeyCode::H | VirtualKeyCode::A => move_player(Direction::West, &mut game.world),
            _ => { return State::Paused }
        }
    }
    State::Running
}
