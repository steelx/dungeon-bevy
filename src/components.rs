use crate::prelude::*;
use bracket_geometry::prelude::Point;

pub struct Materials {
    pub wall_material: Handle<ColorMaterial>,
    pub floor_material: Handle<ColorMaterial>,
    pub player_material: Handle<ColorMaterial>,
    pub monster_material: Handle<ColorMaterial>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

pub fn size_square(x: f32) -> Size {
    Size {width: x, height: x}
}
