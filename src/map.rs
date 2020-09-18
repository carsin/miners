use bracket_lib::prelude::*;

pub enum Direction {
    North, South, East, West
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Empty, Wall
}

pub fn xy_idx(x: usize, y: usize) -> usize {
    // TODO: Scale with map size
    let width = 80;
    (y as usize * width) + x as usize
}

pub fn generate_tile_map(width: usize, height: usize) -> Vec<TileType> {
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
