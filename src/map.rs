use bracket_lib::prelude::*;
use specs::prelude::*;

use std::cmp::{max, min};
use rand::Rng;

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

impl TileType {
    pub fn get_data(self) -> TileData {
        match self {
            TileType::Floor => {
                TileData {
                    glyph: '.',
                    base_fg: RGB::from_f32(0.3, 0.3, 0.3),
                    base_bg: RGB::from_f32(0.1, 0.1, 0.1),
                    blocks_movement: false,
                }
            },

            TileType::Wall => {
                TileData {
                    glyph: '#',
                    base_fg: RGB::from_f32(0.2, 0.2, 0.2),
                    base_bg: RGB::from_f32(0.1, 0.1, 0.1),
                    blocks_movement: true,
                }
            }
        }
    }
}


pub struct TileData {
    pub glyph: char,
    pub base_fg: RGB, // initialize to RGB as we convert to RGBA when rendering to get light level effect
    pub base_bg: RGB, // initialize to RGB as we convert to RGBA when rendering to get light level effect
    pub blocks_movement: bool,
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
    pub tile_entity: Vec<Vec<Entity>>,
    pub tile_blocked: Vec<bool>,
    pub light_levels: Vec<Option<f32>>,
    pub rooms: Vec<Room>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![],
            tile_entity: vec![Vec::new(); width * height],
            tile_blocked: vec![false; width * height],
            light_levels: vec![None; width * height], // initialize all tiles to none (unrevealed)
            rooms: vec![],
            width,
            height,
        }
    }

    pub fn generate_map_rooms_and_corridors(&mut self, room_count: usize, min_room_size: usize, max_room_size: usize) {
        self.tiles = vec![TileType::Wall; self.width * self.height];
        let mut rng = rand::thread_rng();

        for room_num in 0..room_count {
            let mut current_room: Room; // initialize room

            let mut attempt = 0;
            'room_gen: loop {
                attempt += 1;
                // println!("generating new room #{}, attempt {}", room_num, attempt);
                let mut place_room = true;
                let room_w = rng.gen_range(min_room_size..max_room_size);
                let room_h = rng.gen_range(min_room_size..max_room_size);
                let room_x = rng.gen_range(1..self.width - room_w - 1) as i32;
                let room_y = rng.gen_range(1..self.height - room_h - 1) as i32;
                current_room = Room::new(room_x, room_y, room_w, room_h);

                // generate room dimensions
                // loop through other rooms and ensure they dont overlap
                for other_room in self.rooms.iter() {
                    if current_room.overlaps_with(other_room) {
                        println!("failed to generate room #{} on attempt {} with params: w:{}, h:{}, x:{}, y:{}", room_num + 1, attempt, room_w, room_h, room_x, room_y);
                        place_room = false;
                    }
                }

                // if the room didn't intersect with any other rooms, place it and break the generation loop
                if place_room {
                    self.place_room(&current_room);
                    if !self.rooms.is_empty() {
                        let (new_x, new_y) = current_room.center();
                        let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                        // place tunnel to last room, 50/50 if originating horizontally or vertically
                        if rng.gen::<bool>() {
                            self.place_tunnel_horizontal(prev_x, new_x, prev_y);
                            self.place_tunnel_vertical(prev_y, new_y, new_x);
                        } else {
                            self.place_tunnel_vertical(prev_y, new_y, new_x);
                            self.place_tunnel_horizontal(prev_x, new_x, prev_y);
                        }
                    }
                    println!("succeeded in placing room #{} on attempt {} with params: w:{}, h:{}, x:{}, y:{}", room_num + 1, attempt, room_w, room_h, room_x, room_y);
                    self.rooms.push(current_room);
                    break 'room_gen;
                }
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

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.tile_blocked[i] = tile.get_data().blocks_movement;
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;
        // loops through tiles & keeps track of current iteration count in idx
        for (idx, _tile) in self.tiles.iter().enumerate() {
            // render tile if it has been initialized (revealed previously)
            if let Some(light_value) = self.light_levels[idx] {
                let tile_data = self.tiles[idx].get_data();
                let fg = tile_data.base_fg.to_rgba(light_value);
                let bg = tile_data.base_bg.to_rgba(light_value);
                let glyph = tile_data.glyph;

                ctx.print_color(x, y, fg, bg, glyph);
            }

            // Move the coordinates
            x += 1;
            if x >= self.width as i32 {
                x = 0;
                y += 1;
            }
        }
    }

    // clears each tile, but doesn't free up memory and instead keeps memory allocated and ready for data. acquiring new memory is slow!
    pub fn clear_entity_content(&mut self) {
        for entity in self.tile_entity.iter_mut() {
            entity.clear();
        }
    }

    // checks if an exit can be entered
    // fn is_exit_valid(&self, x: i32, y: i32) -> bool {
    //     if x < 1 || x > self.width as i32 - 1 || y < 1 || y > self.height as i32 - 1 { return false }
    //     let idx = self.xy_idx(x, y);
    //     self.tiles[idx as usize] != TileType::Wall
    // }

    // returns index in map array from a coordinate (x, y)
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }
}
