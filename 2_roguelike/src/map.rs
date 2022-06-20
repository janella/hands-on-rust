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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0); // where is this 0 from?
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let idx = get_map_idx(x, y);
                if let Some(tile) = self.tiles.get(idx) {
                    match tile {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            DARK_GRAY,
                            BLACK,
                            to_cp437('#'),
                        ),
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
