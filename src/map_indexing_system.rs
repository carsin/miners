use specs::prelude::*;
use super::{Map, Position, BlocksTile};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile> );


    fn run(&mut self, data : Self::SystemData) {
        let (mut map, position, blockers) = data;

        for (position, _blocker) in (&position, &blockers).join() {
            // block each tile in map vector for each entity that blocks
            let idx = map.xy_idx(position.x, position.y);
            map.tile_blocked[idx] = true;
        }
    }
}
