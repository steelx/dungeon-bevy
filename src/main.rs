mod map;
mod components;
mod map_builder;
mod spawners;
mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub const ARENA_WIDTH: i32 = 20;
    pub const ARENA_HEIGHT: i32 = 20;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawners::*;
    pub use crate::components::*;
    pub use crate::systems::*;
}

use prelude::*;
use rand::prelude::ThreadRng;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::hex("003049").unwrap()))
        .add_resource(WindowDescriptor {
            title: "Dungeon!".to_string(),
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(spawn_player.system())
        .add_system(spawn_monster.system())
        .add_system(spawn_rooms.system())
        .add_system(size_scaling.system())
        .add_system(point_translation.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    commands.insert_resource(Materials {
        wall_material: materials.add(Color::hex("22223b").unwrap().into()),
        player_material: materials.add(Color::hex("d62828").unwrap().into()),
        floor_material: materials.add(Color::hex("ffe8d6").unwrap().into()),
        monster_material: materials.add(Color::hex("ffe800").unwrap().into()),
    });

    let mut rng: ThreadRng = rand::thread_rng();
    let map_builder = MapBuilder::build(&mut rng);
    commands.insert_resource(map_builder);
}
