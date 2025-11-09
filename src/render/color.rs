pub struct RcColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RcColor {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[cfg(feature = "macroquad-renderer")]
impl From<RcColor> for macroquad::color::Color {
    fn from(c: RcColor) -> Self {
        Self::new(c.r, c.g, c.b, c.a)
    }
}
