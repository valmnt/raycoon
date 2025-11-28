use glam::Vec2;

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
