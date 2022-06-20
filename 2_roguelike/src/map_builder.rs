use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        // q: what the heck are we doing here!
        // * is a dereference - t is a reference &TileType
        // we want to write to the REFERENCED VAR and not to the reference
        // a: Vec.iter contents are immutable by default.
        // need to explicitly use iter_mu to mutate contents of Vec
        // Conclusion: Rust enforces explicit mutation. Does not assume
        // you know willy-nilly mutation is bad ("obvious" for experienced devs
        // but not for new devs)
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in &self.rooms {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = get_map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }
}
