use raycoon::ffi::map;

use crate::shared::build_map_ptr;

#[test]
fn ffi_tiles_and_map_creation() {
    let map_ptr = build_map_ptr();
    map::raycoon_map_free(map_ptr);
}
