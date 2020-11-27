use crate::prelude::*;
use bracket_geometry::prelude::Point;

pub const NUM_TILES: usize = (ARENA_WIDTH * ARENA_HEIGHT) as usize;

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

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < ARENA_WIDTH && point.y >= 0 && point.y < ARENA_HEIGHT
    }

    // can_enter_tile if we can walk on a tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    ///try_idx returns tile index if given Point is within bounds
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

/// map_idx for retriving array index of row-first striding vector
pub fn map_idx(x: i32, y: i32) -> usize {
    // vectors are indexed with usize
    ((y * ARENA_WIDTH) + x) as usize
}
