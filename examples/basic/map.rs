use raycoon::engine::{Map, Tiles};
use std::collections::HashSet;

pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 10;
pub const TILE_SIZE: f32 = 38.0;

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

pub fn create_map() -> Map {
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
