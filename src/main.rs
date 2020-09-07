use ggez::event::{self, EventHandler};
use ggez::{conf, graphics, Context, ContextBuilder, GameResult};
use std::path;

struct Game {}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}

fn main() {
    // Make a Context.
    let context_builder = ContextBuilder::new("miners", "miners")
        .window_setup(conf::WindowSetup::default().title("miners"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut ctx, mut event_loop) = context_builder
        .build()
        .expect("Could not create ggez context!");

    let mut my_game = Game::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
