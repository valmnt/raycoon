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

#[no_mangle]
pub extern "C" fn raycoon_cast_result_free(result: RCCast) {
    if result.hits.is_null() {
        return;
    }

    unsafe {
        Vec::from_raw_parts(result.hits, result.len, result.len);
    }
}
