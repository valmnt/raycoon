mod game;
mod map;

use game::Game;
use macroquad::{
    color::BLACK,
    texture::{load_texture, FilterMode},
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};
use map::{create_map, TILE_SIZE};
use raycoon::{
    engine::{self, Screen},
    render::{color::RcColor, Renderer},
    Engine, MacroquadRenderer,
};
use std::f32::consts::PI;

const MOVE_SPEED: f32 = 120.0;
const ROTATION_SPEED: f32 = 3.0;
const SCREEN: engine::Screen = Screen {
    width: 1280,
    height: 720,
};

#[macroquad::main(conf)]
async fn main() {
    let map = create_map();

    let wall_texture = load_texture("assets/wall.png").await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    let engine = Engine::new(map);
    let renderer = MacroquadRenderer::new(wall_texture, 20.0);
    let mut game = Game::new(TILE_SIZE);

    loop {
        clear_background(BLACK);

        game.player
            .handle_key(&engine, get_frame_time(), MOVE_SPEED, ROTATION_SPEED);

        let cast_result = engine.cast_ray(
            game.player.pos,
            game.player.angle,
            PI / 3.0,
            500.0,
            0.8,
            SCREEN,
        );

        let black = RcColor::rgb(0.0, 0.0, 0.0);
        let darkbrown = RcColor::rgb(0.30, 0.25, 0.18);
        renderer.draw_scene(&cast_result, SCREEN, black, darkbrown);

        next_frame().await;
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
