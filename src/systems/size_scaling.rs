use crate::prelude::*;

/// size_scaling
/// if something has a width of 1 in a grid of 40,
/// and the window is 400px across, then it should have a width of 10.
pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    q.iter_mut()
        .for_each(|(size, mut sprite)| {
            sprite.size = Vec2::new(
                size.width / ARENA_WIDTH as f32 * window.width() as f32,
                size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            );
        });
}
