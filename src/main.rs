extern crate bracket_terminal;
extern crate specs;

use bracket_terminal::prelude::*;
use specs::prelude::*;

use components::{Position, Renderable, Moving};
mod components;

bracket_terminal::add_wasm_support!();

struct Game {
    data: World
}

impl Game {
    fn run_systems(&mut self) {
        let mut moving = Moving{};
        moving.run_now(&self.data);

        // Apply changes to World
        self.data.maintain();
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Reset console for next render
        ctx.cls();

        // Update
        self.run_systems();

        // Render
        let positions = self.data.read_storage::<Position>();
        let renderables = self.data.read_storage::<Renderable>();

        for (position, entity) in (&positions, &renderables).join() {
            ctx.print_color(position.x, position.y, entity.fg, entity.bg, entity.glyph);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let mut gamestate: Game = Game {
        data: World::new(),
    };

    gamestate.data.register::<Position>();
    gamestate.data.register::<Renderable>();
    gamestate.data.register::<Moving>();

    // Create player
    gamestate.data.create_entity()
        .with(Position { x: 0, y: 0 })
        .with(Renderable {
            glyph: '@',
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
        })
        .build();

    // Testing NPC's
    for i in 0..10 {
    gamestate.data.create_entity()
        .with(Position { x: i , y: 1 })
        .with(Renderable {
            glyph: '☺',
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .with(Moving{})
        .build();
}

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, gamestate)
}
