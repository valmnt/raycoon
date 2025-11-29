pub mod core;
pub mod map;
pub mod player;
pub mod raycast;

#[repr(C)]
pub struct RCVec2 {
    pub x: f32,
    pub y: f32,
}
