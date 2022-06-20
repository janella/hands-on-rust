use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// q: "smaller types are often faster when you copy them around"
// Example of a smaller type?
// q: == is implemented on PartialEq, what is it?
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = get_map_idx(x, y);
                // Original:
                // match self.tiles[idx] {
                //     TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                //     TileType::Wall => ctx.set(x, y, DARK_GRAY, BLACK, to_cp437('#')),
                // }

                // J: use `get`
                if let Some(tile) = self.tiles.get(idx) {
                    match tile {
                        TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                        TileType::Wall => ctx.set(x, y, DARK_GRAY, BLACK, to_cp437('#')),
                    }
                }
            }
        }
    }

    pub const fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[get_map_idx(point.x, point.y)] == TileType::Floor
    }

    pub const fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(get_map_idx(point.x, point.y))
        } else {
            None
        }
    }
}

pub const fn get_map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
