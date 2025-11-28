use std::collections::HashSet;

#[repr(C)]
pub struct Tiles {
    pub content: Vec<u8>,
    pub blocking: HashSet<u8>,
    pub size: f32,
}

#[repr(C)]
pub struct Map {
    pub tiles: Tiles,
    pub width: usize,
    pub height: usize,
}
