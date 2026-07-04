#[repr(C)]
#[derive(Copy, Clone)]
pub struct RCHit {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
    pub index: usize,
}

#[repr(C)]
pub struct RCCast {
    pub hits: *mut RCHit,
    pub len: usize,
}

/// Frees the hit array owned by a cast result.
///
/// # Parameters
/// - `result`: value returned by [`crate::ffi::core::raycoon_engine_cast_ray`]; a result with a
///   null `hits` pointer is ignored.
///
/// # Safety
/// `result` must be a value returned by [`crate::ffi::core::raycoon_engine_cast_ray`] and freed
/// exactly once; its `hits` pointer must not be reused afterwards.
#[no_mangle]
pub extern "C" fn raycoon_cast_result_free(result: RCCast) {
    if result.hits.is_null() {
        return;
    }

    unsafe {
        Vec::from_raw_parts(result.hits, result.len, result.len);
    }
}
