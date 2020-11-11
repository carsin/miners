use specs::prelude::*;
use super::{Viewshed, Position, Monster, Name};

pub struct MonsterAI {

}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Position>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name> );


    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, mut viewshed, position, monster, name) = data;

        for (viewshed, _position, _monster, name) in (&mut viewshed, &position, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("{}", format!("{}: DON'T KILL ME PLZ IM USELESS", name.name));
            }
            // viewshed.dirty = true;
        }
    }
}
