extern crate bracket_terminal;
extern crate specs;

use bracket_terminal::prelude::*;
use specs::prelude::*;

bracket_terminal::add_wasm_support!();

mod components;

use components::{Position, Renderable};

struct State {
    data: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Hello Bracket World");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let mut gamestate: State = State {
        data: World::new(),
    };

    gamestate.data.register::<Position>();
    gamestate.data.register::<Renderable>();

    gamestate.data.create_entity()
        .with(Position { x: 0, y: 0 })
        .with(Renderable {
            glyph: '@',
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
        })
        .build();

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, gamestate)
}
