use std::ptr;

use crate::engine::{Input, Map, Screen};
use crate::ffi::player::{RCPlayer, RCPlayerInput};
use crate::ffi::raycast::{RCCast, RCHit};
use crate::{engine::Player, ffi::map::RCMap, ffi::RCVec2, Engine};

#[repr(C)]
pub struct RCEngine {
    pub inner: Engine,
}

impl RCEngine {
    /// Wraps an engine built from a `player` and a `map` in an FFI handle.
    ///
    /// # Parameters
    /// - `player`: initial player state.
    /// - `map`: tile map owned by the engine.
    ///
    /// # Returns
    /// An [`RCEngine`] holding the constructed [`Engine`].
    fn new(player: Player, map: Map) -> Self {
        Self {
            inner: Engine::new(player, map),
        }
    }
}

/// Builds a new engine, taking ownership of a player and a map.
///
/// # Parameters
/// - `player`: pointer to an [`RCPlayer`] from [`crate::ffi::player::raycoon_player_new`].
/// - `map`: pointer to an [`RCMap`] from [`crate::ffi::map::raycoon_map_new`].
///
/// # Returns
/// An owning pointer to the new [`RCEngine`], or null if either argument is
/// null. Release it with [`raycoon_engine_free`].
///
/// # Safety
/// `player` and `map` must be valid, non-aliased pointers returned by their
/// respective constructors. They are consumed and must not be used again.
#[no_mangle]
pub extern "C" fn raycoon_engine_new(player: *mut RCPlayer, map: *mut RCMap) -> *mut RCEngine {
    let Some(player) = (unsafe { player.as_mut() }) else {
        return ptr::null_mut();
    };
    let Some(map) = (unsafe { map.as_mut() }) else {
        return ptr::null_mut();
    };

    let RCPlayer { inner: player } = *unsafe { Box::from_raw(player) };
    let RCMap { inner: map } = *unsafe { Box::from_raw(map) };

    let engine = RCEngine::new(player, map);
    Box::into_raw(Box::new(engine))
}

/// Builds a new engine from a player and a map, taking ownership of both.
///
/// Behaves like [`raycoon_engine_new`]; kept as a distinct symbol for callers
/// that construct the engine directly from a map handle.
///
/// # Parameters
/// - `player`: pointer to an [`RCPlayer`] from [`crate::ffi::player::raycoon_player_new`].
/// - `map`: pointer to an [`RCMap`] from [`crate::ffi::map::raycoon_map_new`].
///
/// # Returns
/// An owning pointer to the new [`RCEngine`], or null if either argument is
/// null. Release it with [`raycoon_engine_free`].
///
/// # Safety
/// `player` and `map` must be valid, non-aliased pointers returned by their
/// respective constructors. They are consumed and must not be used again.
#[no_mangle]
pub extern "C" fn raycoon_engine_new_from_map(
    player: *mut RCPlayer,
    map: *mut RCMap,
) -> *mut RCEngine {
    let Some(player) = (unsafe { player.as_mut() }) else {
        return ptr::null_mut();
    };
    let Some(map) = (unsafe { map.as_mut() }) else {
        return ptr::null_mut();
    };

    let RCPlayer { inner: player } = *unsafe { Box::from_raw(player) };
    let RCMap { inner: map } = *unsafe { Box::from_raw(map) };

    let engine = RCEngine::new(player, map);
    Box::into_raw(Box::new(engine))
}

/// Frees an engine previously created by a `raycoon_engine_new*` function.
///
/// # Parameters
/// - `ptr`: engine pointer to free; a null pointer is ignored.
///
/// # Safety
/// `ptr` must have been returned by this crate and not already freed. After
/// this call the pointer is dangling and must not be used again.
#[no_mangle]
pub extern "C" fn raycoon_engine_free(ptr: *mut RCEngine) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

/// Advances the engine's player by one frame using C-friendly input flags.
///
/// Each `RCPlayerInput` field is treated as a boolean (non-zero means pressed)
/// before delegating to [`Engine::update_with_input`].
///
/// # Parameters
/// - `ptr`: engine pointer; a null pointer makes the call a no-op.
/// - `input`: movement/turn flags for this frame.
/// - `delta_time`: time elapsed since the previous frame, in seconds.
/// - `move_speed`: linear speed, in world units per second.
/// - `rotation_speed`: angular speed, in radians per second.
///
/// # Safety
/// `ptr` must be null or a valid pointer returned by this crate and not freed.
#[no_mangle]
pub extern "C" fn raycoon_engine_update_with_input(
    ptr: *mut RCEngine,
    input: RCPlayerInput,
    delta_time: f32,
    move_speed: f32,
    rotation_speed: f32,
) {
    if let Some(engine) = unsafe { ptr.as_mut() } {
        let input = Input {
            turn_left: input.turn_left != 0,
            turn_right: input.turn_right != 0,
            forward: input.forward != 0,
            backward: input.backward != 0,
        };
        engine
            .inner
            .update_with_input(&input, delta_time, move_speed, rotation_speed);
    }
}

/// Casts the scene rays and returns the hits as a C-owned array.
///
/// # Parameters
/// - `ptr`: engine pointer; a null pointer yields an empty [`RCCast`].
/// - `fov`: horizontal field of view, in radians.
/// - `limit`: maximum ray travel distance.
/// - `raystep`: distance between two samples along a ray.
/// - `screen`: target screen dimensions.
///
/// # Returns
/// An [`RCCast`] owning a heap-allocated array of [`RCHit`]. The caller must
/// release it with [`crate::ffi::raycast::raycoon_cast_result_free`].
///
/// # Safety
/// `ptr` must be null or a valid pointer returned by this crate and not freed.
#[no_mangle]
pub extern "C" fn raycoon_engine_cast_ray(
    ptr: *const RCEngine,
    fov: f32,
    limit: f32,
    raystep: f32,
    screen: Screen,
) -> RCCast {
    let Some(engine) = (unsafe { ptr.as_ref() }) else {
        return RCCast {
            hits: std::ptr::null_mut(),
            len: 0,
        };
    };

    let cast = engine.inner.cast_ray(fov, limit, raystep, screen);
    let mut hits: Vec<RCHit> = cast
        .hits
        .into_iter()
        .map(|hit: crate::engine::Hit| RCHit {
            x: hit.x,
            y: hit.y,
            dist: hit.dist,
            index: hit.index,
        })
        .collect();

    let len = hits.len();
    let ptr = hits.as_mut_ptr();
    std::mem::forget(hits);

    RCCast { hits: ptr, len }
}

/// Returns the current player position.
///
/// # Parameters
/// - `ptr`: engine pointer; a null pointer yields `(0.0, 0.0)`.
///
/// # Returns
/// The player position as an [`RCVec2`], in world units.
///
/// # Safety
/// `ptr` must be null or a valid pointer returned by this crate and not freed.
#[no_mangle]
pub extern "C" fn raycoon_engine_player_pos(ptr: *const RCEngine) -> RCVec2 {
    let Some(engine) = (unsafe { ptr.as_ref() }) else {
        return RCVec2 { x: 0.0, y: 0.0 };
    };

    let pos = engine.inner.player.pos;
    RCVec2 { x: pos.x, y: pos.y }
}

/// Returns the current player orientation.
///
/// # Parameters
/// - `ptr`: engine pointer; a null pointer yields `0.0`.
///
/// # Returns
/// The player angle, in radians.
///
/// # Safety
/// `ptr` must be null or a valid pointer returned by this crate and not freed.
#[no_mangle]
pub extern "C" fn raycoon_engine_player_angle(ptr: *const RCEngine) -> f32 {
    let Some(engine) = (unsafe { ptr.as_ref() }) else {
        return 0.0;
    };

    engine.inner.player.angle
}
