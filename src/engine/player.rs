use glam::Vec2;

#[repr(C)]
pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PlayerInput {
    pub turn_left: bool,
    pub turn_right: bool,
    pub forward: bool,
    pub backward: bool,
}
