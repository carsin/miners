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
