#[repr(C)]
#[derive(Copy, Clone)]
pub struct RCHit {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
    pub index: usize,
}
