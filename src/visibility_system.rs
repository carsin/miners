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
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, position) = data;
        for (_entity, viewshed, position) in (&entities, &mut viewshed, &position).join() {
            // update viewshed if game has changed
            if viewshed.dirty {
                viewshed.dirty = false;
                // BUG: viewsheds overwrite each other, so when they're not redrawn the light dissappears
                // Could just fix by changing when the viewshed get dirty
                // how to determine when to get dirty?
                // maybe not
                // Before clearing the viewshed's tiles, loop through this entity's previous iteration
                // of this system and only update the map's light array at those positions.
                // for pos in viewshed.visible_tiles.iter() {
                //     let idx = map.xy_idx(pos.x, pos.y);
                //     map.light_levels[idx] = match map.light_levels[idx] {
                //         None => None, // if tile hasn't been revealed, keep it set to none
                //         Some(_) => Some(BASE_LIGHT_LEVEL), // if it has been previously revealed, make it dark.
                //     };
                // }

                viewshed.visible_tiles.clear();
                let mut shadow_data = shadowcast(Position { x: position.x, y: position.y }, viewshed.range, viewshed.emitter, &*map);
                shadow_data.0.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32 ); // prune everything not within map bounds
                viewshed.visible_tiles = shadow_data.0; // store entities visible tiles (useful for FOV)
                let light_levels = shadow_data.1; // store light levels based on the depth (unused if entity isn't an emitter)

                // set light levels in map
                if let Some(_) = viewshed.emitter {
                    for (i, map_pos) in viewshed.visible_tiles.iter().enumerate() {
                        let idx = map.xy_idx(map_pos.x, map_pos.y); // converts algorithm coords to maps
                        // 9-27-2020: Potential bug fix, instead of clearing everything and then setting values
                        // we could perform a calculation between the previous value and the new calculated value
                        // then check what positions that were calculated in the previous iteration that
                        // weren't recalculated in this iteration
                        map.light_levels[idx] = Some(light_levels[i].unwrap_or(0.0).max(map.light_levels[idx].unwrap_or(0.0)));
                    }
                }
            }
        }
    }
}

// returns two lists:
// one of the positions of all visible tiles from origin,
// and another of those position's light levels (based on algorithm row depth)
fn shadowcast(origin: Position, range: f32, emitter: Option<f32>, map: &Map) -> (Vec<Position>, Vec<Option<f32>>) {
    let mut visible_tiles: Vec<Position> = vec![origin];
    let mut light_levels: Vec<Option<f32>> = vec![];

    if let Some(max_strength) = emitter {
        light_levels.push(Some(max_strength));
    }

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
                    // if an emitter, change light_levels
                    if let Some(max_strength) = emitter {
                        // calculate light level
                        let light_level = max_strength - ((current_row.depth as f32 - 1.0) * max_strength) / range;
                        // println!("{}", light_level);
                        light_levels.push(Some(BASE_LIGHT_LEVEL.max(light_level))); // ensures light level is higher than base light
                    }
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
