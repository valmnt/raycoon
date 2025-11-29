use raycoon::engine::Screen;
use raycoon::ffi::{core, player, raycast, RCVec2};

use crate::shared::{build_map_ptr, TILE_SIZE};

const SCREEN_W: usize = 320;
const SCREEN_H: usize = 200;
const FOV: f32 = std::f32::consts::PI / 3.0;

#[test]
fn ffi_engine_creation_and_cast_ray() {
    let map_ptr = build_map_ptr();

    let player_ptr =
        player::raycoon_player_new(RCVec2 { x: 2.0 * TILE_SIZE, y: TILE_SIZE }, 0.0);
    assert!(!player_ptr.is_null(), "player pointer is null");

    let engine_ptr = core::raycoon_engine_new_from_map(player_ptr, map_ptr);
    assert!(!engine_ptr.is_null(), "engine pointer is null");

    let cast = core::raycoon_engine_cast_ray(
        engine_ptr,
        FOV,
        500.0,
        0.8,
        Screen {
            width: SCREEN_W,
            height: SCREEN_H,
        },
    );

    assert!(!cast.hits.is_null(), "hits pointer is null");
    assert_eq!(cast.len, SCREEN_W);
    let first = unsafe { *cast.hits };
    assert!(first.dist.is_finite());
    assert!(first.dist > 0.0);

    raycast::raycoon_cast_result_free(cast);
    core::raycoon_engine_free(engine_ptr);
}
