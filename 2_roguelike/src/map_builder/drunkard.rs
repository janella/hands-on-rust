use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 40;
pub struct DrunkardsWalkArchitect {}
impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb
    }

    fn drunkard(&mut self, start: &point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
    }
}
