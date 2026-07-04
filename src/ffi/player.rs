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

/// Builds a player handle from a position and an orientation.
///
/// # Parameters
/// - `pos`: initial position, in world units.
/// - `angle`: initial orientation, in radians.
///
/// # Returns
/// An owning pointer to the new [`RCPlayer`]. Release it with
/// [`raycoon_player_free`] unless it is consumed by
/// [`crate::ffi::core::raycoon_engine_new`].
#[no_mangle]
pub extern "C" fn raycoon_player_new(pos: RCVec2, angle: f32) -> *mut RCPlayer {
    let player = Player {
        pos: Vec2::new(pos.x, pos.y),
        angle,
    };

    Box::into_raw(Box::new(RCPlayer { inner: player }))
}

/// Frees a player previously created by [`raycoon_player_new`].
///
/// # Parameters
/// - `ptr`: player pointer to free; a null pointer is ignored.
///
/// # Safety
/// `ptr` must have been returned by this crate and not already freed. A player
/// consumed by [`crate::ffi::core::raycoon_engine_new`] must not be freed here.
#[no_mangle]
pub extern "C" fn raycoon_player_free(ptr: *mut RCPlayer) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
