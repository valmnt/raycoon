mod map;

use glam::vec2;
use macroquad::{
    color::BLACK,
    input::{is_key_down, KeyCode},
    texture::{load_texture, FilterMode},
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};
use map::{create_map, TILE_SIZE};
use raycoon::{
    engine::{self, Player, PlayerInput, Screen},
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
    let player = Player {
        pos: vec2(2.0 * TILE_SIZE, 1.0 * TILE_SIZE),
        angle: 0.0,
    };
    let map = create_map();

    let wall_texture = load_texture("assets/wall.png").await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    let mut engine = Engine::new(player, map);
    let renderer = MacroquadRenderer::new(wall_texture, 20.0);

    loop {
        clear_background(BLACK);

        engine.update_with_input(
            &PlayerInput {
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
