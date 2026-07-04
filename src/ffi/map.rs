use std::{collections::HashSet, ptr, slice};

use crate::engine::{Map, Tiles};

#[repr(C)]
pub struct RCMap {
    pub inner: Map,
}

/// Builds a map from a tile set and grid dimensions, owning the tiles.
///
/// # Parameters
/// - `tiles`: pointer to an [`RCTiles`] from [`raycoon_tiles_new`].
/// - `width`: map width, in tiles.
/// - `height`: map height, in tiles.
///
/// # Returns
/// An owning pointer to the new [`RCMap`], or null if `tiles` is null. Release
/// it with [`raycoon_map_free`].
///
/// # Safety
/// `tiles` must be a valid pointer returned by [`raycoon_tiles_new`]. It is
/// consumed and must not be used again.
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

/// Frees a map previously created by [`raycoon_map_new`].
///
/// # Parameters
/// - `ptr`: map pointer to free; a null pointer is ignored.
///
/// # Safety
/// `ptr` must have been returned by this crate and not already freed. A map
/// consumed by [`crate::ffi::core::raycoon_engine_new`] must not be freed here.
#[no_mangle]
pub extern "C" fn raycoon_map_free(ptr: *mut RCMap) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

#[repr(C)]
pub struct RCTiles {
    pub inner: Tiles,
}

/// Builds a tile set by copying the caller's tile and blocking-id buffers.
///
/// # Parameters
/// - `content_ptr` / `content_len`: tile ids laid out row by row.
/// - `blocking_ptr` / `blocking_len`: tile ids that block movement and rays.
/// - `size`: tile edge length, in world units.
///
/// # Returns
/// An owning pointer to the new [`RCTiles`], or null if a buffer pointer is
/// null while its length is non-zero. Release it with [`raycoon_tiles_free`].
///
/// # Safety
/// Each non-null buffer pointer must be valid for reads of its stated length.
/// The buffers are copied and are not retained after this call.
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

/// Frees a tile set previously created by [`raycoon_tiles_new`].
///
/// # Parameters
/// - `ptr`: tiles pointer to free; a null pointer is ignored.
///
/// # Safety
/// `ptr` must have been returned by this crate and not already freed. Tiles
/// consumed by [`raycoon_map_new`] must not be freed here.
#[no_mangle]
pub extern "C" fn raycoon_tiles_free(ptr: *mut RCTiles) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
