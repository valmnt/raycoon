use raycoon::ffi::{map, player, RCVec2};

pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 10;
pub const TILE_SIZE: f32 = 38.0;

#[rustfmt::skip]
pub const MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
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

pub fn build_map_ptr() -> *mut map::RCMap {
    let blocking = [1u8];
    let tiles_ptr = map::raycoon_tiles_new(
        MAP.as_ptr(),
        MAP.len(),
        blocking.as_ptr(),
        blocking.len(),
        TILE_SIZE,
    );
    assert!(!tiles_ptr.is_null(), "tiles pointer is null");

    let map_ptr = map::raycoon_map_new(tiles_ptr, MAP_WIDTH, MAP_HEIGHT);
    assert!(!map_ptr.is_null(), "map pointer is null");

    map_ptr
}

pub fn build_player_ptr() -> *mut player::RCPlayer {
    let player_ptr = player::raycoon_player_new(
        RCVec2 {
            x: 2.0 * TILE_SIZE,
            y: TILE_SIZE,
        },
        0.0,
    );
    assert!(!player_ptr.is_null(), "player pointer is null");
    player_ptr
}
