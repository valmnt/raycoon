use std::ptr;

use crate::{engine::Map, ffi::tiles::RCTiles};

#[repr(C)]
pub struct RCMap {
    pub inner: Map,
}

#[no_mangle]
pub extern "C" fn raycoon_map_new(tiles: *mut RCTiles, width: usize, height: usize) -> *mut RCMap {
    let Some(tiles) = (unsafe { tiles.as_mut() }) else {
        return ptr::null_mut();
    };

    let tiles = unsafe { Box::from_raw(tiles) };
    let map = Map {
        tiles: tiles.inner,
        width,
        height,
    };

    Box::into_raw(Box::new(RCMap { inner: map }))
}

#[no_mangle]
pub extern "C" fn raycoon_map_free(ptr: *mut RCMap) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
