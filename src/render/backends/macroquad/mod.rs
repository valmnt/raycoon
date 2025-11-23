mod sky_ground;
mod wall_column;

use macroquad::{
    color::WHITE,
    math::vec2,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};
use sky_ground::draw_sky_and_ground;
use wall_column::build_column_from_hit;
use wall_column::RayColumn;

use crate::engine::{CastResult, Screen};
use crate::render::{RcColor, Renderer};

#[cfg(feature = "macroquad-renderer")]
pub struct MacroquadRenderer {
    wall_texture: Texture2D,
    scale: f32,
}

impl MacroquadRenderer {
    pub fn new(wall_texture: Texture2D, scale: f32) -> Self {
        Self {
            wall_texture,
            scale,
        }
    }

    fn draw_column(&self, column: &RayColumn) {
        draw_texture_ex(
            &self.wall_texture,
            column.pos.x,
            column.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(column.src),
                dest_size: Some(column.size),
                ..Default::default()
            },
        );
    }
}

impl Renderer for MacroquadRenderer {
    fn draw_scene(
        &self,
        cast: &CastResult,
        screen: Screen,
        sky_color: RcColor,
        ground_color: RcColor,
    ) {
        let screen_size = vec2(screen.width as f32, screen.height as f32);
        let texture_size = vec2(self.wall_texture.width(), self.wall_texture.height());

        draw_sky_and_ground(screen_size, sky_color, ground_color);

        for hit in cast.hits.iter() {
            let column = build_column_from_hit(hit, screen_size, self.scale, texture_size);
            self.draw_column(&column);
        }
    }
}
