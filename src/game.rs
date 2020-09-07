use ggez::{event, graphics, Context, GameResult};

use super::world;

pub struct Game {
    world: world::GameWorld,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            world: world::GameWorld::new(),
        }
    }
}

impl event::EventHandler for Game {
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
