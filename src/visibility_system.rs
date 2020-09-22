use specs::prelude::*;

use super::{Viewshed, Position, Map, Direction, util, map::TileType};

struct Quadrant {
    origin: Position,
    dir: Direction,
}

impl Quadrant {
    // converts position relative to quadrant to absolute map position
    fn map_pos(&self, old_pos: &Position) -> Position {
        let (x, y) = match self.dir {
            Direction::North => (self.origin.x + old_pos.x, self.origin.y - old_pos.y),
            Direction::South => (self.origin.x + old_pos.x, self.origin.y + old_pos.y),
            Direction::East => (self.origin.x + old_pos.y, self.origin.y + old_pos.x),
            Direction::West => (self.origin.x - old_pos.y, self.origin.y - old_pos.x)
        };

        Position { x, y }
    }
}

struct QuadrantRow {
    depth: i32,
    start_slope: f32,
    end_slope: f32,
}

impl QuadrantRow {
    fn tiles(&self) -> Vec<Position> {
        let mut tiles = vec![];
        let min_x = util::round_tie_up(self.depth as f32 * self.start_slope);
        let max_x = util::round_tie_down(self.depth as f32 * self.end_slope);

        for x in min_x..max_x + 1 {
            tiles.push(Position { x, y: self.depth });
        }

        tiles
    }

    pub fn next(&self) -> Self {
        Self {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }
}

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (ReadExpect<'a, Map>, WriteStorage<'a, Viewshed>, WriteStorage<'a, Position>);

    fn run(&mut self, data : Self::SystemData) {
        let (map, mut viewshed, pos) = data;
        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = fov(Position { x: pos.x, y: pos.y }, viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32 );
        }
    }
}

fn fov(origin: Position, range: usize, map: &Map) -> Vec<Position> {
    let mut visible_tiles: Vec<Position> = vec![origin];

    let dirs = Direction::iterator();
    for dir in dirs {
        let quadrant = Quadrant { origin, dir };

        let first_row = QuadrantRow {
            depth: 1,
            start_slope: -1.0,
            end_slope: 1.0,
        };

        let mut rows = vec![first_row];
        while !rows.is_empty() {
            let mut current_row = rows.pop().unwrap();
            let mut prev_tile: Option<Position> = None;

            let mut prev_tiletype: Option<TileType> = None;
            for curr_tile in current_row.tiles() {
                prev_tiletype = get_tiletype(&map, &prev_tile, &quadrant);
                let curr_tiletype = get_tiletype(&map, &Some(curr_tile), &quadrant);

                if curr_tiletype == Some(TileType::Wall) || is_symmetric(&current_row, &curr_tile) {
                    visible_tiles.push(quadrant.map_pos(&curr_tile)); // reveal
                }

                if prev_tiletype == Some(TileType::Wall) && curr_tiletype == Some(TileType::Floor) {
                    current_row.start_slope = slope(curr_tile);
                }

                if prev_tiletype == Some(TileType::Floor) && curr_tiletype == Some(TileType::Wall) {
                    let mut next_row = current_row.next();
                    next_row.end_slope = slope(curr_tile);
                    rows.push(next_row);
                }

                prev_tile = Some(curr_tile);
                prev_tiletype = get_tiletype(&map, &prev_tile, &quadrant);

            }

            if prev_tiletype == Some(TileType::Floor) {
                rows.push(current_row.next());
            }

            if current_row.depth >= range as i32 {
                break;
            }
        }
    }

    visible_tiles
}

fn is_symmetric(row: &QuadrantRow, tile: &Position) -> bool {
    tile.x as f32 >= row.depth as f32 * row.start_slope && tile.x as f32 <= row.depth as f32 * row.end_slope
}

fn slope(tile: Position) -> f32 {
    (2 * tile.x - 1) as f32 / (2 * tile.y) as f32
}

fn get_tiletype(map: &Map, tile: &Option<Position>, quadrant: &Quadrant) -> Option<TileType> {
    match tile {
        None => None,
        Some(pos) => {
            let pos = quadrant.map_pos(pos);
            let idx = map.xy_idx(pos.x, pos.y);
            Some(map.tiles[idx])
        }
    }
}
