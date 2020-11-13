use specs::prelude::*;
use super::{Viewshed, Position, Monster, Name, MeleeAttacking, RunState};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Position>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, MeleeAttacking>
                      );


    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, player_entity, runstate, entities, mut viewshed, position, monster, name, mut melee_attack) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, viewshed, _position, _monster, name) in (&entities, &mut viewshed, &position, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("{}", format!("{}: DON'T KILL ME PLZ IM USELESS", name.name));
                melee_attack.insert(entity, MeleeAttacking { target: *player_entity }).expect("Adding attack target failed.");
            }
            // viewshed.dirty = true;
        }
    }
}
