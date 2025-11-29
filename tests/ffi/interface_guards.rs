use std::ptr;

use raycoon::engine::Screen;
use raycoon::ffi::{core, map, player, raycast, RCVec2};

use crate::shared::{build_map_ptr, build_player_ptr, TILE_SIZE};

#[test]
fn ffi_player_creation_and_free_are_null_safe() {
    let player_ptr = player::raycoon_player_new(
        RCVec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        0.0,
    );
    assert!(!player_ptr.is_null());
    player::raycoon_player_free(player_ptr);
    player::raycoon_player_free(ptr::null_mut());
}

#[test]
fn ffi_engine_new_rejects_null_inputs_and_free_accepts_null() {
    let engine_null_player = core::raycoon_engine_new(ptr::null_mut(), ptr::null_mut());
    assert!(engine_null_player.is_null());

    let map_ptr = build_map_ptr();
    let engine_null_player_only = core::raycoon_engine_new(ptr::null_mut(), map_ptr);
    assert!(engine_null_player_only.is_null());
    map::raycoon_map_free(map_ptr);

    let player_ptr = build_player_ptr();
    let engine_null_map_only = core::raycoon_engine_new(player_ptr, ptr::null_mut());
    assert!(engine_null_map_only.is_null());
    player::raycoon_player_free(player_ptr);

    core::raycoon_engine_free(ptr::null_mut());
}

#[test]
fn ffi_engine_new_from_map_happy_path_consumes_inputs() {
    let map_ptr = build_map_ptr();
    let player_ptr = build_player_ptr();

    let engine_ptr = core::raycoon_engine_new_from_map(player_ptr, map_ptr);
    assert!(!engine_ptr.is_null());

    core::raycoon_engine_free(engine_ptr);
}

#[test]
fn ffi_engine_new_happy_path_consumes_inputs() {
    let map_ptr = build_map_ptr();
    let player_ptr = build_player_ptr();

    let engine_ptr = core::raycoon_engine_new(player_ptr, map_ptr);
    assert!(!engine_ptr.is_null());

    core::raycoon_engine_free(engine_ptr);
}

#[test]
fn ffi_engine_new_from_map_rejects_null_inputs() {
    let map_ptr = build_map_ptr();
    let engine_with_null_player = core::raycoon_engine_new_from_map(ptr::null_mut(), map_ptr);
    assert!(engine_with_null_player.is_null());
    map::raycoon_map_free(map_ptr);

    let player_ptr = build_player_ptr();
    let engine_with_null_map =
        core::raycoon_engine_new_from_map(player_ptr, ptr::null_mut());
    assert!(engine_with_null_map.is_null());
    player::raycoon_player_free(player_ptr);
}

#[test]
fn ffi_engine_cast_ray_handles_null_engine_and_free_result_null_safe() {
    let cast = core::raycoon_engine_cast_ray(
        ptr::null(),
        std::f32::consts::FRAC_PI_2,
        1.0,
        1.0,
        Screen {
            width: 10,
            height: 10,
        },
    );

    assert!(cast.hits.is_null());
    assert_eq!(cast.len, 0);

    raycast::raycoon_cast_result_free(cast);
}

#[test]
fn ffi_engine_update_and_player_access_are_null_safe() {
    let input = player::RCPlayerInput {
        turn_left: 0,
        turn_right: 0,
        forward: 0,
        backward: 0,
    };

    core::raycoon_engine_update_with_input(ptr::null_mut(), input, 0.0, 0.0, 0.0);

    let pos = core::raycoon_engine_player_pos(ptr::null());
    assert_eq!(pos.x, 0.0);
    assert_eq!(pos.y, 0.0);

    let angle = core::raycoon_engine_player_angle(ptr::null());
    assert_eq!(angle, 0.0);
}

#[test]
fn ffi_cast_result_free_accepts_null_hits() {
    let result = raycast::RCCast {
        hits: ptr::null_mut(),
        len: 0,
    };
    raycast::raycoon_cast_result_free(result);
}
