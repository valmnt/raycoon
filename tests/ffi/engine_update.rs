use raycoon::ffi::{core, player, RCVec2};

use crate::shared::{build_map_ptr, TILE_SIZE};

#[test]
fn ffi_update_with_input_smoke() {
    let map_ptr = build_map_ptr();

    let player_ptr =
        player::raycoon_player_new(RCVec2 { x: 2.0 * TILE_SIZE, y: TILE_SIZE }, 0.0);
    assert!(!player_ptr.is_null());

    let engine_ptr = core::raycoon_engine_new_from_map(player_ptr, map_ptr);
    assert!(!engine_ptr.is_null());

    let input = player::RCPlayerInput {
        turn_left: 0,
        turn_right: 0,
        forward: 1,
        backward: 0,
    };

    core::raycoon_engine_update_with_input(engine_ptr, input, 1.0 / 60.0, 120.0, 3.0);

    core::raycoon_engine_free(engine_ptr);
}
