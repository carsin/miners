extern crate bracket_terminal;
extern crate specs;

use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

mod components;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Hello Bracket World");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let gs: State = State {};

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, gs)
}
