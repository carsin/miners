use specs::prelude::*;
use super::{Viewshed, Position, Monster};

pub struct MonsterAI {

}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( WriteStorage<'a, Viewshed>, ReadStorage<'a, Position>, ReadStorage<'a, Monster> );


    fn run(&mut self, data : Self::SystemData) {
        let (mut viewshed, position, monster) = data;

        for (viewshed, position, _monster) in (&mut viewshed, &position, &monster).join() {
            // viewshed.dirty = true;
        }
    }
}
