mod map;
mod components;
mod map_builder;
mod spawners;
mod systems;

pub mod prelude {
    pub use bevy::prelude::*;
    pub const ARENA_WIDTH: i32 = 100;
    pub const ARENA_HEIGHT: i32 = 100;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawners::*;
    pub use crate::components::*;
    pub use crate::systems::*;
}

use prelude::*;
use rand::prelude::ThreadRng;
use bevy::app::startup_stage;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::hex("003049").unwrap()))
        .add_resource(WindowDescriptor {
            title: "Dungeon!".to_string(),
            width: 800_f32,
            height: 800_f32,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_rooms.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_player.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, spawn_monster.system())
        .add_system(size_scaling.system())
        .add_system(point_translation.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Materials {
        wall_material: materials.add(Color::hex("22223b").unwrap().into()),
        player_material: materials.add(Color::hex("06d6a0").unwrap().into()),
        floor_material: materials.add(Color::hex("ffe8d6").unwrap().into()),
        monster_material: materials.add(Color::hex("d62828").unwrap().into()),
    });

    let mut rng: ThreadRng = rand::thread_rng();
    let map_builder = MapBuilder::build(&mut rng);
    commands.insert_resource(map_builder);
}
