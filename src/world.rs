use specs::{
    Builder, World, WorldExt,
};

use super::components::{Renderable, Position, Wall, Player};

pub struct GameWorld {
    world: World,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            world: World::new()
        }
    }

    pub fn register_components(&mut self) {
        self.world.register::<Position>();
        self.world.register::<Renderable>();
        self.world.register::<Wall>();
        self.world.register::<Player>();
    }

    pub fn create_player(&mut self, position: Position) {
        self.world.create_entity()
            .with(Position { z: 1, ..position })
            .with(Renderable {
                path: String::from("player.png")
            })
            .build();
    }
}
