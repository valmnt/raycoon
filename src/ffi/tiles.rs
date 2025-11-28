use std::{collections::HashSet, ptr, slice};

use crate::engine::Tiles;

#[repr(C)]
pub struct RCTiles {
    pub inner: Tiles,
}

#[no_mangle]
pub extern "C" fn raycoon_tiles_new(
    content_ptr: *const u8,
    content_len: usize,
    blocking_ptr: *const u8,
    blocking_len: usize,
    size: f32,
) -> *mut RCTiles {
    if (content_ptr.is_null() && content_len > 0) || (blocking_ptr.is_null() && blocking_len > 0) {
        return ptr::null_mut();
    }

    let content = if content_len == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(content_ptr, content_len) }.to_vec()
    };

    let blocking: HashSet<u8> = if blocking_len == 0 {
        HashSet::new()
    } else {
        unsafe { slice::from_raw_parts(blocking_ptr, blocking_len) }
            .iter()
            .copied()
            .collect()
    };

    let tiles = Tiles {
        content,
        blocking,
        size,
    };

    Box::into_raw(Box::new(RCTiles { inner: tiles }))
}

#[no_mangle]
pub extern "C" fn raycoon_tiles_free(ptr: *mut RCTiles) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
