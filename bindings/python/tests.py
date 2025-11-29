"""
Minimal sanity test for the Python FFI wrapper.
"""

import math
import sys
from pathlib import Path

repo_root = Path(__file__).resolve().parent
bindings_root = repo_root / "bindings"
candidate_paths = [
    bindings_root,
    bindings_root / "python",
]
for path in candidate_paths:
    if path.exists() and str(path) not in sys.path:
        sys.path.append(str(path))

from raycoon import (
    PlayerInput,
    Screen,
    Vec2,
    cast_ray,
    free_engine,
    make_engine,
    make_map,
    make_player,
    make_tiles,
    update_with_input,
    engine_player_pos,
    engine_player_angle,
)

MAP_WIDTH, MAP_HEIGHT, TILE_SIZE = 10, 10, 38.0
MAP = bytes(
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 1, 1, 1, 0, 1, 1, 0, 1,
        1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
        1, 0, 1, 0, 1, 0, 1, 0, 1, 1,
        1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        1, 1, 1, 0, 1, 1, 1, 1, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
        1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ]
)
BLOCKING = bytes([1])
SCREEN = Screen(width=1280, height=720)
FOV, LIMIT, RAYSTEP = math.pi / 3.0, 500.0, 0.8

def run_cast() -> None:
    player = make_player(Vec2(2.0 * TILE_SIZE, 1.0 * TILE_SIZE), 0.0)
    tiles = make_tiles(MAP, BLOCKING, TILE_SIZE)
    map_ptr = make_map(tiles, MAP_WIDTH, MAP_HEIGHT)
    engine = make_engine(player, map_ptr)
    
    hits = cast_ray(engine, FOV, LIMIT, RAYSTEP, SCREEN)
    
    pos = engine_player_pos(engine)
    angle = engine_player_angle(engine)

    player_input = PlayerInput(turn_left=1, turn_right=0, forward=1, backward=0)
    update_with_input(engine, player_input, delta_time=1.0 / 60.0, move_speed=120.0, rotation_speed=3.0)
    
    pos_after = engine_player_pos(engine)
    angle_after = engine_player_angle(engine)

    assert angle_after < angle
    assert pos_after.x > pos.x

    assert len(hits) == SCREEN.width
    first = hits[0]
    assert math.isfinite(first.dist) and first.dist > 0.0
    assert -0.5 <= first.x < 0.5
    assert -0.5 <= first.y < 0.5
    assert first.index == 0
    assert math.isclose(first.dist, 0.6928, rel_tol=1e-3, abs_tol=1e-4)

    free_engine(engine)


def test_python_ffi_cast_ray():
    run_cast()


if __name__ == "__main__":
    run_cast()
    print("python ffi test passed")
