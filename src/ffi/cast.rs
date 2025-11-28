use crate::ffi::hit::RCHit;

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
