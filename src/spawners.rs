use crate::prelude::*;
use bracket_geometry::prelude::Point;

pub fn spawn_player(commands: &mut Commands, map_builder: Res<MapBuilder>, materials: Res<Materials>) {
    // SpriteBundle is a Bundle of components,
    // that means we get a ** Transform ** component,
    // among a bunch of others (Sprite, Mesh, Draw, Rotation, Scale, etc).
    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            material: materials.player_material.clone(),
            ..Default::default()
        })
        .with(Player)
        .with(map_builder.player_start)
        .with(size_square(0.8));
}

pub fn spawn_monster(commands: &mut Commands, map_builder: Res<MapBuilder>, materials: Res<Materials>) {
    map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    material: materials.monster_material.clone(),
                    ..Default::default()
                })
                .with(Enemy)
                .with(pos)
                .with(size_square(0.8));
        });
}

pub fn spawn_rooms(commands: &mut Commands, windows: Res<Windows>, map_builder: Res<MapBuilder>, materials: Res<Materials>) {
    // dbg!(&map_builder.rooms);
    let window = windows.get_primary().unwrap();

    for y in 0..window.height() as i32 {
        for x in 0..window.width() as i32 {
            let pos = Point::new(x, y);
            let idx = map_idx(x, y);
            if map_builder.map.in_bounds(pos) {
                match map_builder.map.tiles[idx] {
                    TileType::Floor => {
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                                material: materials.floor_material.clone(),
                                ..Default::default()
                            })
                            .with(TileType::Floor)
                            .with(pos)
                            .with(size_square(1.));
                    },
                    TileType::Wall => {
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                                material: materials.wall_material.clone(),
                                ..Default::default()
                            })
                            .with(TileType::Wall)
                            .with(pos)
                            .with(size_square(1.));
                    }
                }
            }
        }
    }
}
