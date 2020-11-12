use specs::{WorldExt, World, prelude::*};
use bracket_terminal::prelude::*;
use super::{Direction, Player, Position, Map, util, Viewshed, Game, State, CombatStats, MeleeAttacking};

pub fn move_player(dir: Direction, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut player = world.write_storage::<Player>();
    let mut viewsheds = world.write_storage::<Viewshed>();
    let mut combat_stats = world.write_storage::<CombatStats>();
    let mut melee_attack = world.write_storage::<MeleeAttacking>();

    let map = world.fetch::<Map>();
    let entities = world.entities();

    // convert move direction to dx & dy
    let (delta_x, delta_y) = match dir {
        Direction::North => { (0, -1) },
        Direction::South => { (0, 1) }
        Direction::East => { (1, 0) }
        Direction::West => { (-1, 0) }
    };

    // run ecs system
    for (entity, _player, pos, viewshed) in (&entities, &mut player, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width as i32 - 1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height as i32 - 1 { return; } // don't try to attack outside map
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // check if there is an entity in destination tile & attack it
        for potential_target in map.tile_entity[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                melee_attack.insert(entity, MeleeAttacking { target: *potential_target }).expect("Adding attack target failed.");
                return; // don't move if player attacked attacked
            }
        }

        // move if the destination tile isn't blocked
        if !map.tile_blocked[destination_idx] {
            pos.x = util::clamp(pos.x + delta_x, 0, (map.width - 1) as i32);
            pos.y = util::clamp(pos.y + delta_y, 0, (map.height - 1) as i32);

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
