use glam::{vec2, Vec2 as GlamVec2};
use macroquad::{
    color::{BLACK, DARKBROWN, WHITE},
    input::{is_key_down, KeyCode},
    prelude::{vec2 as mq_vec2, Color, Rect, Vec2},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, load_texture, DrawTextureParams, FilterMode, Texture2D},
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};
use raycoon::{
    engine::{self, column_from_hit, Hit, Input, Map, Player, Projection, Screen, Tiles},
    Engine,
};
use std::{collections::HashSet, f32::consts::PI};

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;
const TILE_SIZE: f32 = 38.0;

#[rustfmt::skip]
const MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 1, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 1,
    1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
    1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

const MOVE_SPEED: f32 = 120.0;
const ROTATION_SPEED: f32 = 3.0;
const SCREEN: engine::Screen = Screen {
    width: 1280,
    height: 720,
};

struct RayColumn {
    pos: Vec2,
    size: Vec2,
    src: Rect,
}

struct MacroquadRenderer {
    wall_texture: Texture2D,
    scale: f32,
}

impl MacroquadRenderer {
    fn new(wall_texture: Texture2D, scale: f32) -> Self {
        Self {
            wall_texture,
            scale,
        }
    }

    fn build_column_from_hit(
        &self,
        hit: &Hit,
        screen: Vec2,
        scale: f32,
        texture_size: Vec2,
    ) -> RayColumn {
        let proj: Projection = column_from_hit(
            hit,
            GlamVec2::new(screen.x, screen.y),
            scale,
            GlamVec2::new(texture_size.x, texture_size.y),
        );

        RayColumn {
            pos: mq_vec2(proj.screen_pos.x, proj.screen_pos.y),
            size: mq_vec2(proj.screen_size.x, proj.screen_size.y),
            src: Rect {
                x: proj.tex_pos.x,
                y: proj.tex_pos.y,
                w: proj.tex_size.x,
                h: proj.tex_size.y,
            },
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

    fn draw_sky_and_ground(&self, screen: Vec2, sky_color: Color, ground_color: Color) {
        draw_rectangle(0.0, 0.0, screen.x, screen.y / 2.0, sky_color);
        draw_rectangle(0.0, screen.y / 2.0, screen.x, screen.y / 2.0, ground_color);
    }

    fn draw_scene(
        &self,
        cast: &raycoon::engine::Scanline,
        screen: Screen,
        sky_color: Color,
        ground_color: Color,
    ) {
        let screen_size = mq_vec2(screen.width as f32, screen.height as f32);
        let texture_size = mq_vec2(self.wall_texture.width(), self.wall_texture.height());

        self.draw_sky_and_ground(screen_size, sky_color, ground_color);

        for hit in cast.hits.iter() {
            let column = self.build_column_from_hit(hit, screen_size, self.scale, texture_size);
            self.draw_column(&column);
        }
    }
}

#[macroquad::main(conf)]
async fn main() {
    let player = Player {
        pos: vec2(2.0 * TILE_SIZE, 1.0 * TILE_SIZE),
        angle: 0.0,
    };
    let map = create_map();

    let wall_texture_path = format!("{}/assets/wall.png", env!("CARGO_MANIFEST_DIR"));
    let wall_texture = load_texture(&wall_texture_path).await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    let mut engine = Engine::new(player, map);
    let renderer = MacroquadRenderer::new(wall_texture, 20.0);

    loop {
        clear_background(BLACK);

        engine.update_with_input(
            &Input {
                turn_left: is_key_down(KeyCode::Left),
                turn_right: is_key_down(KeyCode::Right),
                forward: is_key_down(KeyCode::Up),
                backward: is_key_down(KeyCode::Down),
            },
            get_frame_time(),
            MOVE_SPEED,
            ROTATION_SPEED,
        );

        let cast_result = engine.cast_ray(PI / 3.0, 500.0, 0.8, SCREEN);

        renderer.draw_scene(&cast_result, SCREEN, BLACK, DARKBROWN);

        next_frame().await;
    }
}

fn create_map() -> Map {
    let mut blocking_tile = HashSet::new();
    blocking_tile.insert(1);

    let tiles = Tiles {
        content: MAP.to_vec(),
        blocking: blocking_tile,
        size: TILE_SIZE,
    };

    Map {
        tiles,
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Raycoon Basic Demo".into(),
        window_width: SCREEN.width as i32,
        window_height: SCREEN.height as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
