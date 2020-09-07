use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};

use super::components;

pub struct GameWorld {
    world: World,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            world: World::new()
        }
    }

    pub fn register_components(&mut self) {
        self.world.register::<components::Position>();
        self.world.register::<components::Renderable>();
        self.world.register::<components::Wall>();
        self.world.register::<components::Player>();
    }
}
