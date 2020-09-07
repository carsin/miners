extern crate bracket_terminal;
extern crate specs;

use bracket_terminal::prelude::*;
use specs::prelude::*;

use components::{Position, Renderable, Moving, Player, Direction, move_player};
mod components;

bracket_terminal::add_wasm_support!();

struct Game {
    world: World
}

impl Game {
    fn run_systems(&mut self) {
        let mut moving = Moving{};
        moving.run_now(&self.world);

        // Apply changes to World
        self.world.maintain();
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::W => move_player(Direction::North, &mut self.world),
                VirtualKeyCode::S => move_player(Direction::South, &mut self.world),
                VirtualKeyCode::A => move_player(Direction::East, &mut self.world),
                VirtualKeyCode::D => move_player(Direction::West, &mut self.world),
                _ => {}
            }
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Reset console for next render
        ctx.cls();

        self.handle_input(ctx);

        // Update
        self.run_systems();

        // Render
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, entity) in (&positions, &renderables).join() {
            ctx.print_color(position.x, position.y, entity.fg, entity.bg, entity.glyph);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let mut game: Game = Game {
        world: World::new(),
    };

    game.world.register::<Position>();
    game.world.register::<Renderable>();
    game.world.register::<Player>();
    game.world.register::<Moving>();

    // Create player
    game.world.create_entity()
        .with(Position { x: 8, y: 8 })
        .with(Renderable {
            glyph: '@',
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
        })
        .with(Player{})
        .build();

    // Testing NPC's
    for i in 0..10 {
    game.world.create_entity()
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
    main_loop(context, game)
}
