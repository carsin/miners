extern crate ggez;
extern crate specs;

use ggez::{conf, event, ContextBuilder};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};
use std::path;

mod game;

fn main() {
    // Make a Context.
    let context_builder = ContextBuilder::new("miners", "miners")
        .window_setup(conf::WindowSetup::default().title("miners"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("../resources"));

    let (mut ctx, mut event_loop) = context_builder
        .build()
        .expect("Could not create ggez context!");

    let mut my_game = game::Game::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
