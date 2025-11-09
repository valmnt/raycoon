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
