use std::cmp::{max, min};
use components::Position;
use rand::Rng;

use bracket_lib::prelude::*;
use specs::prelude::*;
use super::components;

#[derive(Copy, Clone)]
pub enum Direction {
    North, South, East, West
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::East, Direction::South, Direction::West].iter().copied()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, Wall
}

#[derive(Debug)]
pub struct Room {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: usize, h: usize) -> Self {
        Self {x1: x, y1: y, x2: x + w as i32, y2: y + h as i32}
    }

    pub fn overlaps_with(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![],
            rooms: vec![],
            width,
            height,
        }
    }

    pub fn generate_map_rooms_and_corridors(&mut self, max_rooms: usize, min_room_size: usize, max_room_size: usize) {
        self.tiles = vec![TileType::Wall; self.width * self.height];

        let mut rng = rand::thread_rng();

        for _ in 0..max_rooms {
            let room_w = rng.gen_range(min_room_size, max_room_size);
            let room_h = rng.gen_range(min_room_size, max_room_size);
            let room_x = rng.gen_range(0, self.width - room_w - 1) as i32;
            let room_y = rng.gen_range(0, self.height - room_h - 1) as i32;

            let new_room = Room::new(room_x, room_y, room_w, room_h);

            let mut room_fits = true;
            for other_room in self.rooms.iter() {
                if new_room.overlaps_with(other_room) {
                    room_fits = false;
                }
            }

            if room_fits {
                self.place_room(&new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();

                    // Generates a random bool, so 50/50
                    if rng.gen::<bool>() {
                        self.place_tunnel_horizontal(prev_x, new_x, prev_y);
                        self.place_tunnel_vertical(prev_y, new_y, new_x);
                    } else {
                        self.place_tunnel_vertical(prev_y, new_y, new_x);
                        self.place_tunnel_horizontal(prev_x, new_x, prev_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

    }

    fn place_room(&mut self, room: &Room) {
        let mut pos: usize;
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                pos = self.xy_idx(x, y);
                self.tiles[pos] = TileType::Floor;
            }
        }
    }

    fn place_tunnel_horizontal(&mut self, x1: i32, x2: i32, y: i32) {
        let mut pos: usize;
        for x in min(x1, x2)..=max(x1, x2) {
            pos = self.xy_idx(x, y);
            if pos > 0 && pos < self.width * self.height {
                self.tiles[pos] = TileType::Floor;
            }
        }
    }

    fn place_tunnel_vertical(&mut self, y1: i32, y2: i32, x: i32) {
        let mut pos: usize;
        for y in min(y1, y2)..=max(y1, y2) {
            pos = self.xy_idx(x, y);
            if pos > 0 && pos < self.width * self.height {
                self.tiles[pos] = TileType::Floor;
            }
        }
    }

    pub fn render(&self, world: &World, ctx: &mut BTerm) {
        let mut viewsheds = world.write_storage::<components::Viewshed>();
        let mut players = world.write_storage::<components::Player>();

        for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
            let mut y = 0;
            let mut x = 0;
            for tile in self.tiles.iter() {
                // Render a tile depending upon the tile type
                let pos = Position { x, y };
                if viewshed.visible_tiles.contains(&pos) {
                    match tile {
                        TileType::Floor => {
                            ctx.print_color(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), '.');
                        }

                        TileType::Wall => {
                            ctx.print_color(x, y, RGB::from_f32(0.3, 0.3, 0.3), RGB::from_f32(0., 0., 0.), '#');
                        }
                    }
                }

                // Move the coordinates
                x += 1;
                if x >= self.width as i32 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }
}
