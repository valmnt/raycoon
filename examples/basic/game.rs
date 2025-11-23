use glam::{vec2, Vec2};
use macroquad::input::{is_key_down, KeyCode};
use raycoon::Engine;

pub struct Game {
    pub player: Player,
}

impl Game {
    pub fn new(tile_size: f32) -> Self {
        Self {
            player: Player {
                pos: vec2(2.0 * tile_size, 1.0 * tile_size),
                angle: 0.0,
            },
        }
    }
}

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

impl Player {
    pub fn handle_key(
        &mut self,
        engine: &Engine,
        delta_time: f32,
        move_speed: f32,
        rotation_speed: f32,
    ) {
        let dir = vec2(self.angle.cos(), self.angle.sin());
        let mut delta = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            self.angle -= rotation_speed * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += rotation_speed * delta_time;
        }

        if is_key_down(KeyCode::Up) {
            delta += dir;
        }
        if is_key_down(KeyCode::Down) {
            delta -= dir;
        }

        if delta.length_squared() > 0.0 {
            delta = delta.normalize() * move_speed * delta_time;
            engine.move_with_collision(&mut self.pos, delta);
        }
    }
}
