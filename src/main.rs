use bracket_terminal::prelude::*;
use specs::prelude::*;
use rand::Rng;

use components::*;
use map::{Direction, Map};
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use damage_system::DamageSystem;
use gamelog::GameLog;

mod components;
mod map;
mod player;
mod util;
mod visibility_system;
mod monster_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;
mod gamelog;

const GAME_WIDTH: usize = 80;
const GAME_HEIGHT: usize = 50;

const MAP_WIDTH: usize = GAME_WIDTH;
const MAP_HEIGHT: usize = GAME_HEIGHT - 7;

const BASE_LIGHT_LEVEL: f32 = 0.0;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub struct Game {
    pub world: World,
}

impl Game {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.world);
        let mut monsters = MonsterAI{};
        monsters.run_now(&self.world);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.world);
        let mut melee_combat = MeleeCombatSystem{};
        melee_combat.run_now(&self.world);
        let mut damage = DamageSystem{};
        damage.run_now(&self.world);

        // Apply changes to World
        self.world.maintain();
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Reset console for next render
        ctx.cls();
        let mut new_run_state;

        // get current run state
        {
            let run_state = self.world.fetch::<RunState>();
            new_run_state = *run_state;
        }

        // run according to run state
        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            },

            RunState::AwaitingInput => {
                new_run_state = player::input(self, ctx);
            },

            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            },

            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            },
        }

        // update run state
        {
            let mut run_writer = self.world.write_resource::<RunState>();
            *run_writer = new_run_state;
        }

        // Remove dead entites
        damage_system::remove_dead(&mut self.world);

        // Render map
        let map = self.world.fetch::<Map>();
        map.render(ctx);

        // Render entities
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, entity) in (&positions, &renderables).join() {
            let idx = map.xy_idx(position.x, position.y);
            if let Some(light_level) = map.light_levels[idx] {
                // TODO: Change to if in player FOV
                // only render if entity is lit
                if light_level > BASE_LIGHT_LEVEL {
                    let fg = entity.fg.to_rgba(light_level);
                    let bg = entity.bg.to_rgba(light_level);
                    ctx.print_color(position.x, position.y, fg, bg, entity.glyph);
                }
            }
        }
        ctx.print_color(0, 0, RGB::named(WHITE), RGB::named(BLACK), &format!("{} fps", ctx.fps as u32)); // Render FPS
        gui::draw_ui(&self.world, ctx);
    }
}

// Options: Kjammer_16x16, Md_16x16, Yayo16x16, Zilk16x16
bracket_terminal::embedded_resource!(TILE_FONT, "../resources/Zilk_16x16.png");

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "resources/Zilk_16x16.png");
    let mut rng = rand::thread_rng();
    let context = BTermBuilder::new()
        .with_tile_dimensions(16, 16)
        .with_dimensions(GAME_WIDTH, GAME_HEIGHT)
        .with_font("Zilk_16x16.png", 16, 16)
        .with_title("miners !dwmf")
        .with_simple_console(GAME_WIDTH, GAME_HEIGHT, "Zilk_16x16.png")
        // .with_automatic_console_resize(true)
        .build()?;

    let mut game: Game = Game {
        world: World::new(),
    };

    game.world.register::<Position>();
    game.world.register::<Renderable>();
    game.world.register::<Player>();
    game.world.register::<Viewshed>();
    game.world.register::<Monster>();
    game.world.register::<Name>();
    game.world.register::<CombatStats>();
    game.world.register::<BlocksTile>();
    game.world.register::<MeleeAttacking>();
    game.world.register::<SufferDamage>();

    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);

    let room_count: usize = 10;
    let min_room_size: usize = 4;
    let max_room_size: usize = 10;

    map.generate_map_rooms_and_corridors(room_count, min_room_size, max_room_size);

    // Create player
    let (player_x, player_y) = map.rooms[0].center();
    let player_entity = game.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: '☺',
            fg: RGB::from_f32(0.9, 0.9, 0.9),
            bg: RGB::from_f32(0.1, 0.1, 0.1),
        })
        .with(Player {})
        .with(Viewshed { visible_tiles: vec![], light_levels: vec![], emitter: Some(1.0), range: 5.0, dirty: true })
        .with(Name { name: String::from("Player") })
        .with(CombatStats { max_hp: 30, hp: 30, armor: 0, damage: 5 })
        .build();

    let mut zombie_count = 0;
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        // 50/50 to spawn torch or monster
        if rng.gen::<bool>() {
            // spawn monster
            zombie_count += 1;
            game.world.create_entity()
                .with(Position { x: x - 1, y: y + 1 })
                .with(Renderable {
                    glyph: 'z',
                    fg: RGB::from_f32(0.1, 0.5, 0.1),
                    bg: RGB::from_f32(0.1, 0.1, 0.1),
                })
                .with(Viewshed { visible_tiles: vec![], light_levels: vec![], emitter: None, range: 1.0, dirty: true })
                .with(Monster {})
                .with(Name { name: format!("Zombie #{}", zombie_count) })
                .with(BlocksTile {})
                .with(CombatStats { max_hp: 10, hp: 10, armor: 0, damage: 1 })
                .build();
        } else {
            game.world.create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    glyph: 'i',
                    fg: RGB::from_f32(1.0, 0.6, 0.0),
                    bg: RGB::from_f32(0.1, 0.1, 0.1),
                })
                .with(Name { name: String::from("Torch")})
                .with(Viewshed { visible_tiles: vec![], light_levels: vec![], emitter: Some(0.6), range: 6.0, dirty: true })
                .build();
        }
    }

    game.world.insert(GameLog { entries: vec![String::from("Welcome to mine.rs")] });
    game.world.insert(map);
    game.world.insert(player_entity);
    game.world.insert(Position::new(player_x, player_y));
    game.world.insert(RunState::PreRun);

    // Call into bracket_terminal to run the main loop. This handles rendering, and calls back into State's tick function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, game)
}
