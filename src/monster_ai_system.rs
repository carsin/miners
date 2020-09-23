use specs::prelude::*;
use super::{Viewshed, Position, Monster};

pub struct MonsterAI {

}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, Viewshed>, ReadStorage<'a, Position>, ReadStorage<'a, Monster> );


    fn run(&mut self, data : Self::SystemData) {
        let (viewshed, position, monster) = data;

        for (viewshed, position, _monster) in (&viewshed, &position, &monster).join() {

        }
    }
}
