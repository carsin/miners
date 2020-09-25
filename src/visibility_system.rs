use specs::prelude::*;

use super::{Viewshed, Position, Map, Direction, util, map::TileType, BASE_LIGHT_LEVEL};

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
    type SystemData = (WriteExpect<'a, Map>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>);

    // runs for entities with viewshed & position components
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, position) = data;

        // reset map light levels
        // currently this runs everytime the system runs, which is currently fine
        // since the player moving is the only time the system is updated, but this will need to be changed
        // store some sort of boolean in map instead of in each entity?
        // but when entities need to recalculate they need to keep track of their dirtiness as well to be efficient
        for tile in map.light_levels.iter_mut() {
            match tile {
                None => *tile = None, // if tile hasn't been revealed, keep it set to none
                Some(_) => *tile = Some(BASE_LIGHT_LEVEL), // if it has been previously revealed, make it dark.
            }
        }

        for (_entity, viewshed, position) in (&entities, &mut viewshed, &position).join() {
            // update viewshed if game has changed
            if viewshed.dirty {

                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                let mut shadow_data = shadowcast(Position { x: position.x, y: position.y }, viewshed.strength, &*map);
                shadow_data.0.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32 ); // prune everything not within map bounds
                viewshed.visible_tiles = shadow_data.0; // store entities visible tiles (useful for FOV)
                let light_levels = shadow_data.1; // store light levels based on the depth (unused if entity isn't an emitter)

                // set light levels in map
                if viewshed.emits_light {
                    for (i, relative_pos) in viewshed.visible_tiles.iter().enumerate() {
                        let idx = map.xy_idx(relative_pos.x, relative_pos.y); // converts algorithm coords to maps
                        map.light_levels[idx] = light_levels[i];
                    }
                }
            }
        }
    }
}

// returns two lists:
// one of the positions of all visible tiles from origin,
// and another of those position's light levels (based on algorithm row depth)
// I should seperate these
fn shadowcast(origin: Position, strength: f32, map: &Map) -> (Vec<Position>, Vec<Option<f32>>) {
    let mut visible_tiles: Vec<Position> = vec![origin];
    let mut light_levels: Vec<Option<f32>> = vec![Some(1.0)];

    // iterates through all 4 directions (NESW)
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
                    // Add to visible tiles
                    visible_tiles.push(quadrant.map_pos(&curr_tile));
                    // calculate light level
                    let light_level = 1.0 - ((current_row.depth as f32 - 1.0) / strength) - BASE_LIGHT_LEVEL;
                    light_levels.push(Some(BASE_LIGHT_LEVEL.max(light_level))); // ensures light level is higher than base light
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

            if current_row.depth >= strength as i32 {
                break;
            }
        }
    }
    (visible_tiles, light_levels)
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
