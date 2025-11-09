pub mod color;

#[cfg(feature = "macroquad-renderer")]
mod backends;

use crate::{
    engine::{CastResult, Screen},
    render::color::RcColor,
};

#[cfg(feature = "macroquad-renderer")]
pub use backends::macroquad::MacroquadRenderer;

pub trait Renderer {
    fn draw_scene(
        &self,
        cast: &CastResult,
        screen: Screen,
        sky_color: RcColor,
        ground_color: RcColor,
    );
}
