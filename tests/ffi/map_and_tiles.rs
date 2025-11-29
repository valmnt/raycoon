use raycoon::ffi::map;

use std::ptr;

use crate::shared::{build_map_ptr, TILE_SIZE};

#[test]
fn ffi_tiles_and_map_creation() {
    let map_ptr = build_map_ptr();
    map::raycoon_map_free(map_ptr);
}

#[test]
fn ffi_tiles_rejects_null_content_with_len() {
    let tiles_ptr = map::raycoon_tiles_new(ptr::null(), 1, ptr::null(), 0, TILE_SIZE);
    assert!(tiles_ptr.is_null());
}

#[test]
fn ffi_tiles_rejects_null_blocking_with_len() {
    let tiles_ptr = map::raycoon_tiles_new(ptr::null(), 0, ptr::null(), 1, TILE_SIZE);
    assert!(tiles_ptr.is_null());
}

#[test]
fn ffi_map_new_rejects_null_tiles() {
    let map_ptr = map::raycoon_map_new(ptr::null_mut(), 1, 1);
    assert!(map_ptr.is_null());
}

#[test]
fn ffi_map_free_accepts_null() {
    map::raycoon_map_free(ptr::null_mut());
}

#[test]
fn ffi_tiles_free_accepts_null() {
    map::raycoon_tiles_free(ptr::null_mut());
}
