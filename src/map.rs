use std::cmp::max;
use std::cmp::min;

use bracket_lib::prelude::*;

pub enum Direction {
    North, South, East, West
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Empty, Wall
}

pub struct Rect {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize,) -> Self {
        Self {x1: x, y1: y, x2: x + w, y2: y + h }
    }

    pub fn overlaps_with(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![],
            width,
            height,
        }
    }

    pub fn generate_map_rooms_and_corridors(&mut self) {
        self.tiles = vec![TileType::Wall; self.width * self.height];
        let room1 = Rect::new(1, 1, 10, 10);
        let room2 = Rect::new(31, 1, 10, 10);

        self.place_room(&room1);
        self.place_room(&room2);

        self.place_tunnel_horizontal(11, 30, 1);
    }

    fn place_room(&mut self, room: &Rect) {
        let mut pos: usize;
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                pos = self.xy_idx(x, y);
                self.tiles[pos] = TileType::Empty;
            }
        }
    }

    fn place_tunnel_horizontal(&mut self, x1: usize, x2: usize, y: usize) {
        let mut pos: usize;
        for x in min(x1, x2)..=max(x1, x2) {
            pos = self.xy_idx(x, y);
            if pos > 0 && pos < self.width * self.height {
                self.tiles[pos] = TileType::Empty;
            }
        }
    }

    fn place_tunnel_vertical(&mut self, y1: usize, y2: usize, y: usize) {
        let mut pos: usize;
        for x in min(y1, y2)..=max(y1, y2) {
            pos = self.xy_idx(x, y);
            if pos > 0 && pos < self.width * self.height {
                self.tiles[pos] = TileType::Empty;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;
        for tile in self.tiles.iter() {
            // Render a tile depending upon the tile type
            match tile {
                TileType::Empty => {
                    ctx.print_color(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), '.');
                }

                TileType::Wall => {
                    ctx.print_color(x, y, RGB::from_f32(0.3, 0.3, 0.3), RGB::from_f32(0., 0., 0.), '#');
                }
            }

            // Move the coordinates
            x += 1;
            if x >= self.width {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}
