# Changelog

## 0.0.4 - 2025-11-30
- Removed the Python wrapper/bindings and now validate the C ABI through Rust integration tests.
- Reorganized engine and FFI modules (columns/raycast split, map constructors) to align the ABI surface with engine structure and safer ownership transfer.
- Hardened the basic example’s texture loading by resolving the wall texture via `CARGO_MANIFEST_DIR` to avoid relative-path failures.

## 0.0.3 - 2025-11-29
- Added backend-agnostic player input abstraction so engine APIs stay decoupled from specific libraries (#5).
- Refactored core into a render-agnostic engine with a C ABI FFI layer and official Python binding; removed embedded Macroquad renderer (#10).
- Moved column projection and player movement/rotation into the engine, leaving renderers as thin input/output adapters (#11).

## 0.0.2 - 2025-11-23
- Remove heavy assets from the published crate package to keep it lightweight.
- Clarify installation/feature flag usage in the README.
- Minor metadata/docs cleanups.

## 0.0.1 - 2025-11-23
- Initial release of the minimal raycasting engine (ray casting + tile collisions).
- Added Macroquad rendering backend behind the optional `macroquad-renderer` feature.
- Included playable basic example (`cargo run --example basic --features macroquad-renderer`).
- Completed crate metadata (MIT license, docs.rs, repository).
