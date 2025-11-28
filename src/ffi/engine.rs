use std::ptr;

use crate::engine::{Map, PlayerInput, Screen};
use crate::ffi::cast::RCCast;
use crate::ffi::hit::RCHit;
use crate::ffi::player::{RCPlayer, RCPlayerInput};
use crate::{engine::Player, ffi::map::RCMap, Engine};

#[repr(C)]
pub struct RCEngine {
    pub inner: Engine,
}

impl RCEngine {
    fn new(player: Player, map: Map) -> Self {
        Self {
            inner: Engine::new(player, map),
        }
    }
}

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

#[no_mangle]
pub extern "C" fn raycoon_engine_free(ptr: *mut RCEngine) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

#[no_mangle]
pub extern "C" fn raycoon_engine_update_with_input(
    ptr: *mut RCEngine,
    input: RCPlayerInput,
    delta_time: f32,
    move_speed: f32,
    rotation_speed: f32,
) {
    if let Some(engine) = unsafe { ptr.as_mut() } {
        let input = PlayerInput {
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
        .map(|hit: crate::engine::RayHit| RCHit {
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
