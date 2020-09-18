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

pub fn generate_map_test(width: usize, height: usize) -> Vec<TileType> {
    let mut map = vec![TileType::Empty; width * height];

    // Make map edges walls
    for x in 0..width {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, height - 1)] = TileType::Wall;
    }

    for y in 0..height {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(width - 1, y)] = TileType::Wall;
    }

    map
}

pub fn generate_map_rooms_and_corridors(width: usize, height: usize) -> Vec<TileType> {
    let mut map = vec![TileType::Wall; width * height];

    let room1 = Rect::new(1, 1, 10, 10);
    let room2 = Rect::new(12, 1, 10, 10);

    place_room(&room1, &mut map);
    place_room(&room2, &mut map);

    map
}

fn place_room(room: &Rect, map: &mut [TileType]) {
    for y in room.y1..room.y2 {
        for x in room.x1..room.x2 {
            map[xy_idx(x, y)] = TileType::Empty;
        }
    }
}

pub fn render_map(ctx: &mut BTerm, map: &[TileType]) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Empty => {
                ctx.print_color(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), '.');
            }

            TileType::Wall => {
                ctx.print_color(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), '#');
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

pub fn xy_idx(x: usize, y: usize) -> usize {
    // TODO: Scale with map size
    let width = 80;
    (y as usize * width) + x as usize
}
