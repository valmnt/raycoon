use glam::{vec2, Vec2};

use crate::engine::{
    player::Player,
    raycast::{Hit, Scanline},
    Input,
};

use super::{Map, Screen};

pub struct Engine {
    pub player: Player,
    map: Map,
}

impl Engine {
    pub fn new(player: Player, map: Map) -> Self {
        assert!(
            map.tiles.content.len() == map.width * map.height,
            "Invalid tiles length: got {}, expected {} (width * height)",
            map.tiles.content.len(),
            map.width * map.height,
        );

        Self { player, map }
    }

    pub fn cast_ray(&self, fov: f32, limit: f32, raystep: f32, screen: Screen) -> Scanline {
        let mut hits = Vec::new();

        for ray_i in 0..screen.width {
            let ray_angle =
                self.player.angle - fov / 2.0 + fov * (ray_i as f32 / screen.width as f32);

            let mut current_distance = 0.0;

            while current_distance < limit {
                let current_dist_x = self.player.pos.x + current_distance * ray_angle.cos();
                let current_dist_y = self.player.pos.y + current_distance * ray_angle.sin();

                if self.hit_tile(vec2(current_dist_x, current_dist_y)) {
                    let angle_diff = ray_angle - self.player.angle;
                    let distance = current_distance.max(0.0001);
                    let distance_corrected = distance * angle_diff.cos().abs();

                    let tile_x = current_dist_x / self.map.tiles.size;
                    let tile_y = current_dist_y / self.map.tiles.size;

                    let hit_x = tile_x - (tile_x + 0.5).floor();
                    let hit_y = tile_y - (tile_y + 0.5).floor();

                    hits.push(Hit {
                        x: hit_x,
                        y: hit_y,
                        dist: distance_corrected,
                        index: ray_i,
                    });

                    break;
                }

                current_distance += raystep;
            }
        }

        Scanline { hits }
    }

    pub fn update_with_input(
        &mut self,
        input: &Input,
        delta_time: f32,
        move_speed: f32,
        rotation_speed: f32,
    ) {
        if input.turn_left {
            self.player.angle -= rotation_speed * delta_time;
        }
        if input.turn_right {
            self.player.angle += rotation_speed * delta_time;
        }

        let dir = Vec2::new(self.player.angle.cos(), self.player.angle.sin());
        let mut delta = Vec2::ZERO;

        if input.forward {
            delta += dir;
        }
        if input.backward {
            delta -= dir;
        }

        if delta.length_squared() > 0.0 {
            let delta = delta.normalize() * move_speed * delta_time;
            self.move_with_collision(delta);
        }
    }

    fn move_with_collision(&mut self, delta: Vec2) {
        let next = self.player.pos + delta;

        let x = vec2(next.x, self.player.pos.y);
        if !self.hit_tile(x) {
            self.player.pos.x = next.x;
        }

        let y = vec2(self.player.pos.x, next.y);
        if !self.hit_tile(y) {
            self.player.pos.y = next.y;
        }
    }

    fn pixel_to_tile(&self, pos: Vec2) -> Option<(usize, usize)> {
        if pos.x < 0.0 || pos.y < 0.0 {
            return None;
        }
        let x = (pos.x / self.map.tiles.size) as usize;
        let y = (pos.y / self.map.tiles.size) as usize;
        if x < self.map.width && y < self.map.height {
            Some((x, y))
        } else {
            None
        }
    }

    fn hit_tile(&self, pos: Vec2) -> bool {
        self.pixel_to_tile(pos)
            .map(|(x, y)| {
                self.map
                    .tiles
                    .blocking
                    .contains(&self.map.tiles.content[y * self.map.width + x])
            })
            .unwrap_or(true)
    }
}
