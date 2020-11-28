use crate::prelude::*;
use bracket_geometry::prelude::Point;

pub fn point_translation(
    windows: Res<Windows>,
    mut q: Query<Without<TileType, (&Point, &mut Transform)>>,
    mut map: Query<With<TileType, (&Point, &mut Transform)>>,
) {

    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();

    q.iter_mut()
        .for_each(|(pos, mut transform)| {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                1.0
            );
        });

    map.iter_mut()
        .for_each(|(pos, mut transform)| {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0
            );
        });
}
