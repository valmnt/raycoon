pub struct Hit {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
    pub index: usize,
}

pub struct Scanline {
    pub hits: Vec<Hit>,
}
