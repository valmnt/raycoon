use crate::engine::{project_hit_to_column, ColumnProjection, RayHit};
use glam::Vec2 as GlamVec2;
use macroquad::prelude::{vec2, Rect, Vec2};

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
    let proj: ColumnProjection = project_hit_to_column(
        hit,
        GlamVec2::new(screen.x, screen.y),
        scale,
        GlamVec2::new(texture_size.x, texture_size.y),
    );

    RayColumn {
        pos: vec2(proj.screen_pos.x, proj.screen_pos.y),
        size: vec2(proj.screen_size.x, proj.screen_size.y),
        src: Rect {
            x: proj.tex_pos.x,
            y: proj.tex_pos.y,
            w: proj.tex_size.x,
            h: proj.tex_size.y,
        },
    }
}
