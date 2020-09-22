use bracket_terminal::prelude::*;
use specs::prelude::*;

use components::{Player, Position, Renderable, Viewshed};
use map::{Direction, Map};
use visibility_system::VisibilitySystem;

mod components;
mod map;
mod player;
mod util;
mod visibility_system;

struct Game {
    world: World
}

impl Game {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.world);

        // Apply changes to World
        self.world.maintain();
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::W => player::move_player(Direction::North, &mut self.world),
                VirtualKeyCode::S => player::move_player(Direction::South, &mut self.world),
                VirtualKeyCode::A => player::move_player(Direction::East, &mut self.world),
                VirtualKeyCode::D => player::move_player(Direction::West, &mut self.world),
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

        // Render map
        let map = self.world.fetch::<Map>();
        map.render(&self.world, ctx);

        // Render entities
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, entity) in (&positions, &renderables).join() {
            ctx.print_color(position.x, position.y, entity.fg, entity.bg, entity.glyph);
        }

        // Render FPS
        ctx.print_centered(0, &format!("{} fps", ctx.fps as u32));
    }
}

fn main() -> BError {
    // TODO: better game sizing
    let context = BTermBuilder::simple80x50()
        .with_title("miners")
        .build()?;

    let mut game: Game = Game {
        world: World::new(),
    };

    game.world.register::<Position>();
    game.world.register::<Renderable>();
    game.world.register::<Player>();
    game.world.register::<Viewshed>();
    // game.world.register::<Moving>();

    let mut map = Map::new(80, 50);
    map.generate_map_rooms_and_corridors(10, 6, 10);
    let (player_x, player_y) = map.rooms[0].center();
    game.world.insert(map);

    // Create player
    game.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: '☺',
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Viewshed { visible_tiles : vec![], range: 8 })
        .build();

    // Testing NPCs
    // for i in 0..10 {
    //     game.world.create_entity()
    //         .with(Position { x: i , y: 1 })
    //         .with(Renderable {
    //             glyph: '☺',
    //             fg: RGB::named(RED),
    //             bg: RGB::named(BLACK),
    //         })
    //         .with(Moving{})
    //         .build();
    // }

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, game)
}
