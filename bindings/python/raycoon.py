"""
Minimal ctypes wrapper for python around the raycoon C FFI.
"""

import sys
from ctypes import (
    POINTER,
    Structure,
    c_float,
    c_size_t,
    c_uint8,
    c_void_p,
    cdll,
)
from pathlib import Path

def _default_lib_path() -> Path:
    ext = {"linux": ".so", "darwin": ".dylib"}.get(sys.platform, ".dll")
    return Path(__file__).resolve().parents[2] / "target" / "release" / f"libraycoon{ext}"

def load_library(path: Path | None = None):
    lib_path = path or _default_lib_path()
    if not lib_path.exists():
        raise FileNotFoundError(f"raycoon shared library not found at {lib_path}")
    return cdll.LoadLibrary(str(lib_path))

_lib = load_library()

class Vec2(Structure):
    _fields_ = [("x", c_float), ("y", c_float)]

class PlayerInput(Structure):
    _fields_ = [
        ("turn_left", c_uint8),
        ("turn_right", c_uint8),
        ("forward", c_uint8),
        ("backward", c_uint8),
    ]

class Screen(Structure):
    _fields_ = [("width", c_size_t), ("height", c_size_t)]

class RCHit(Structure):
    _fields_ = [("x", c_float), ("y", c_float), ("dist", c_float), ("index", c_size_t)]

class RCCast(Structure):
    _fields_ = [("hits", POINTER(RCHit)), ("len", c_size_t)]

RCTiles = c_void_p
RCMap = c_void_p
RCPlayer = c_void_p
RCEngine = c_void_p

_lib.raycoon_tiles_new.argtypes = [
    POINTER(c_uint8),
    c_size_t,
    POINTER(c_uint8),
    c_size_t,
    c_float,
]
_lib.raycoon_tiles_new.restype = RCTiles
_lib.raycoon_tiles_free.argtypes = [RCTiles]

_lib.raycoon_map_new.argtypes = [RCTiles, c_size_t, c_size_t]
_lib.raycoon_map_new.restype = RCMap
_lib.raycoon_map_free.argtypes = [RCMap]

_lib.raycoon_player_new.argtypes = [Vec2, c_float]
_lib.raycoon_player_new.restype = RCPlayer
_lib.raycoon_player_free.argtypes = [RCPlayer]

_lib.raycoon_engine_new_from_map.argtypes = [RCPlayer, RCMap]
_lib.raycoon_engine_new_from_map.restype = RCEngine
_lib.raycoon_engine_free.argtypes = [RCEngine]
_lib.raycoon_engine_player_pos.argtypes = [RCEngine]
_lib.raycoon_engine_player_pos.restype = Vec2
_lib.raycoon_engine_player_angle.argtypes = [RCEngine]
_lib.raycoon_engine_player_angle.restype = c_float

_lib.raycoon_engine_update_with_input.argtypes = [
    RCEngine,
    PlayerInput,
    c_float,
    c_float,
    c_float,
]

_lib.raycoon_engine_cast_ray.argtypes = [
    RCEngine,
    c_float,
    c_float,
    c_float,
    Screen,
]
_lib.raycoon_engine_cast_ray.restype = RCCast
_lib.raycoon_cast_result_free.argtypes = [RCCast]

def make_tiles(content: bytes, blocking: bytes, size: float) -> RCTiles:
    content_arr = (c_uint8 * len(content))(*content) if content else None
    blocking_arr = (c_uint8 * len(blocking))(*blocking) if blocking else None
    return _lib.raycoon_tiles_new(
        content_arr,
        len(content),
        blocking_arr,
        len(blocking),
        c_float(size),
    )

def make_map(tiles: RCTiles, width: int, height: int) -> RCMap:
    return _lib.raycoon_map_new(tiles, width, height)

def make_player(pos: Vec2, angle: float):
    return _lib.raycoon_player_new(pos, c_float(angle))

def make_engine(player_ptr: RCPlayer, map_ptr: RCMap) -> RCEngine:
    return _lib.raycoon_engine_new_from_map(player_ptr, map_ptr)

def update_with_input(
    engine: RCEngine,
    player_input: PlayerInput,
    delta_time: float,
    move_speed: float,
    rotation_speed: float,
):
    _lib.raycoon_engine_update_with_input(
        engine,
        player_input,
        c_float(delta_time),
        c_float(move_speed),
        c_float(rotation_speed),
    )

def cast_ray(
    engine: RCEngine,
    fov: float,
    limit: float,
    raystep: float,
    screen: Screen,
):
    result = _lib.raycoon_engine_cast_ray(
        engine,
        c_float(fov),
        c_float(limit),
        c_float(raystep),
        screen,
    )
    hits: list[RCHit] = []
    if result.hits and result.len > 0:
        raw_hits = [result.hits[i] for i in range(result.len)]
        hits = [RCHit(h.x, h.y, h.dist, h.index) for h in raw_hits]
    _lib.raycoon_cast_result_free(result)
    return hits

def free_engine(engine: RCEngine):
    _lib.raycoon_engine_free(engine)

def free_player(player_ptr: RCPlayer):
    _lib.raycoon_player_free(player_ptr)

def free_map(map_ptr: RCMap):
    _lib.raycoon_map_free(map_ptr)

def free_tiles(tiles_ptr: RCTiles):
    _lib.raycoon_tiles_free(tiles_ptr)

def engine_player_pos(engine: RCEngine):
    return _lib.raycoon_engine_player_pos(engine)

def engine_player_angle(engine: RCEngine):
    return _lib.raycoon_engine_player_angle(engine)
