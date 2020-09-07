use ggez::{event, graphics, Context, GameResult};

use super::world::GameWorld;

pub struct GameState {
    world: GameWorld,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            world: GameWorld::new(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        {}
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx)
    }
}
