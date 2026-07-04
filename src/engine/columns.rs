use glam::Vec2;

use crate::engine::raycast::Hit;

pub struct Projection {
    pub screen_pos: Vec2,
    pub screen_size: Vec2,
    pub tex_pos: Vec2,
    pub tex_size: Vec2,
}

/// Projects a ray [`Hit`] into the vertical wall column to draw for it.
///
/// The wall face coordinate (`hit.x` or `hit.y`, whichever is dominant) is
/// mapped to a texture column, and the on-screen height is derived from the
/// corrected hit distance.
///
/// # Parameters
/// - `hit`: the ray hit to project.
/// - `screen_size`: render target size, in pixels.
/// - `scale`: wall height scaling factor.
/// - `texture_size`: wall texture size, in pixels.
///
/// # Returns
/// A [`Projection`] describing where to draw the column on screen and which
/// texture slice to sample.
pub fn column_from_hit(hit: &Hit, screen_size: Vec2, scale: f32, texture_size: Vec2) -> Projection {
    let mut texture_u = hit.x;
    if hit.y.abs() > hit.x.abs() {
        texture_u = hit.y;
    }

    if texture_u < 0.0 {
        texture_u += 1.0;
    }

    let texture_x = texture_u * texture_size.x;

    let column_height = (screen_size.y * scale) / hit.dist;
    let column_y = screen_size.y * 0.5 - column_height * 0.5;
    let column_x = hit.index as f32;

    Projection {
        screen_pos: Vec2::new(column_x, column_y),
        screen_size: Vec2::new(1.0, column_height),
        tex_pos: Vec2::new(texture_x, 0.0),
        tex_size: Vec2::new(1.0, texture_size.y),
    }
}
