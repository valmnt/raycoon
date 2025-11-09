use macroquad::prelude::*;

use crate::render::RcColor;

pub(crate) fn draw_sky_and_ground(screen: Vec2, sky_color: RcColor, ground_color: RcColor) {
    draw_rectangle(0.0, 0.0, screen.x, screen.y / 2.0, Color::from(sky_color));
    draw_rectangle(
        0.0,
        screen.y / 2.0,
        screen.x,
        screen.y / 2.0,
        Color::from(ground_color),
    );
}
