use bracket_terminal::prelude::*;
use specs::prelude::*;

use components::{Player, Position, Renderable, Viewshed, Monster};
use map::{Direction, Map};
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;

mod components;
mod map;
mod player;
mod util;
mod visibility_system;
mod monster_ai_system;

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Paused,
    Running
}


pub struct Game {
    pub world: World,
    pub state: State
}

impl Game {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.world);
        let mut monsters = MonsterAI{};
        monsters.run_now(&self.world);

        // Apply changes to World
        self.world.maintain();
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Reset console for next render
        ctx.cls();

        // Turn based updating
        if self.state == State::Running {
            self.run_systems();
            self.state = State::Paused;
        } else {
            self.state = player::input(self, ctx);
        }

        // Render map
        let map = self.world.fetch::<Map>();
        map.render(&self.world, ctx);

        // Render entities
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, entity) in (&positions, &renderables).join() {
            let idx = map.xy_idx(position.x, position.y);
            if map.visible_tiles[idx] {
                ctx.print_color(position.x, position.y, entity.fg, entity.bg, entity.glyph);
            }
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
        state: State::Running,
    };

    game.world.register::<Position>();
    game.world.register::<Renderable>();
    game.world.register::<Player>();
    game.world.register::<Viewshed>();
    game.world.register::<Monster>();

    let mut map = Map::new(80, 50);
    map.generate_map_rooms_and_corridors(30, 3, 8);

    let (player_x, player_y) = map.rooms[0].center();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        game.world.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: 'g',
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Viewshed { visible_tiles : vec![], range: 5, dirty: true })
            .with(Monster {})
            .build();
    }

    game.world.insert(map);

    // Create player
    game.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: '☺',
            fg: RGB::from_f32(0.0, 1.0, 1.0),
            bg: RGB::from_f32(0.2, 0.2, 0.2),
        })
        .with(Player {})
        .with(Viewshed { visible_tiles : vec![], range: 5, dirty: true })
        .build();

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, game)
}
