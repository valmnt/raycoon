use crate::engine::RayHit;
use macroquad::prelude::*;

pub(crate) struct RayColumn {
    pub(crate) pos: Vec2,
    pub(crate) size: Vec2,
    pub(crate) src: Rect,
}

pub(crate) fn build_column_from_hit(
    hit: &RayHit,
    screen: Vec2,
    scale: f32,
    texture_size: Vec2,
) -> RayColumn {
    let mut texture_u = hit.x;
    if hit.y.abs() > hit.x.abs() {
        texture_u = hit.y;
    }

    if texture_u < 0.0 {
        texture_u += 1.0;
    }

    let texture_x = texture_u * texture_size.x;

    let column_height = (screen.y * scale) / hit.dist;
    let column_y = screen.y / 2.0 - column_height / 2.0;
    let column_x = hit.index as f32;

    let pos = vec2(column_x, column_y);
    let size = vec2(1.0, column_height);
    let src = Rect {
        x: texture_x,
        y: 0.0,
        w: 1.0,
        h: texture_size.y,
    };

    RayColumn { pos, size, src }
}
