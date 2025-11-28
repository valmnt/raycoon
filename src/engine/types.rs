use glam::Vec2;

pub struct RayHit {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
    pub index: usize,
}

pub struct CastResult {
    pub hits: Vec<RayHit>,
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
}

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

pub struct PlayerInput {
    pub turn_left: bool,
    pub turn_right: bool,
    pub forward: bool,
    pub backward: bool,
}
