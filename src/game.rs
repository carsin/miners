use ggez::{event, graphics::{self, DrawParam, Image}, nalgebra, Context, GameResult};
use specs::{
    ReadStorage, RunNow, System, join::Join
};
use super::world::GameWorld;
use super::components::{Renderable, Position};

const TILE_WIDTH: f32 = 8.0;

pub struct GameState {
    pub world: GameWorld,
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
        // Render game state
        {
            let mut renderer = RenderSystem { ctx };
            renderer.run_now(&self.world.data);
        }

        Ok(())
    }
}


pub struct RenderSystem<'a> {
    ctx: &'a mut Context
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.ctx, graphics::WHITE);

        // Get all the renderables with their positions and sort by the position z
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.ctx, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(nalgebra::Point2::new(x, y));
            graphics::draw(self.ctx, &image, draw_params).expect("expected render");
        }

        graphics::present(self.ctx);
    }
}
