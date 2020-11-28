use crate::prelude::*;
use rand::Rng;
use bracket_geometry::prelude::{Rect, Point};
use rand::rngs::ThreadRng;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles
            .iter_mut()
            .for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut ThreadRng) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.gen_range(1, ARENA_WIDTH - 10),
                rng.gen_range(1, ARENA_HEIGHT - 10),
                rng.gen_range(2, 10),
                rng.gen_range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < ARENA_WIDTH && p.y > 0 && p.y < ARENA_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
            }

            self.rooms.push(room);
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut ThreadRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.gen_range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            }
        }
    }

    pub fn build(rng: &mut ThreadRng) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start = map_builder.rooms[0].center();

        map_builder
    }
}
