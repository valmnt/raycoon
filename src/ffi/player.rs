use glam::Vec2;

use crate::{engine::Player, ffi::RCVec2};

#[repr(C)]
pub struct RCPlayer {
    pub inner: Player,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RCPlayerInput {
    pub turn_left: u8,
    pub turn_right: u8,
    pub forward: u8,
    pub backward: u8,
}

#[no_mangle]
pub extern "C" fn raycoon_player_new(pos: RCVec2, angle: f32) -> *mut RCPlayer {
    let player = Player {
        pos: Vec2::new(pos.x, pos.y),
        angle,
    };

    Box::into_raw(Box::new(RCPlayer { inner: player }))
}

#[no_mangle]
pub extern "C" fn raycoon_player_free(ptr: *mut RCPlayer) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
