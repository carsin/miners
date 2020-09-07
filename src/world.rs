use specs::{
    Builder, World, WorldExt,
};

use super::components::{Renderable, Position, Wall, Player};

pub struct GameWorld {
    pub data: World,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            data: World::new()
        }
    }

    pub fn register_components(&mut self) {
        self.data.register::<Position>();
        self.data.register::<Renderable>();
        self.data.register::<Wall>();
        self.data.register::<Player>();
    }

    pub fn create_player(&mut self, position: Position) {
        self.data.create_entity()
            .with(Position { z: 1, ..position })
            .with(Renderable {
                path: String::from("/player.png/")
            })
            .build();
    }

    pub fn create_wall(&mut self, position: Position) {
        self.data.create_entity()
            .with(Position { z: 1, ..position })
            .with(Renderable {
                path: String::from("/wall.png/")
            })
            .build();
    }
}
