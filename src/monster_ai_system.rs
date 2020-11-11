use specs::prelude::*;
use super::{Viewshed, Position, Monster};

pub struct MonsterAI {

}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Position>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster> );


    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, mut viewshed, position, monster) = data;

        for (viewshed, _position, _monster) in (&mut viewshed, &position, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("I SEE U HA");
            }
            // viewshed.dirty = true;
        }
    }
}
