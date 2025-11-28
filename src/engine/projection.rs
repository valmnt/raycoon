use glam::Vec2;

pub struct ColumnProjection {
    pub screen_pos: Vec2,
    pub screen_size: Vec2,
    pub tex_pos: Vec2,
    pub tex_size: Vec2,
}

pub struct RayHit {
    pub x: f32,
    pub y: f32,
    pub dist: f32,
    pub index: usize,
}

pub struct CastResult {
    pub hits: Vec<RayHit>,
}

pub fn project_hit_to_column(
    hit: &RayHit,
    screen_size: Vec2,
    scale: f32,
    texture_size: Vec2,
) -> ColumnProjection {
    let mut texture_u = hit.x;
    if hit.y.abs() > hit.x.abs() {
        texture_u = hit.y;
    }

    if texture_u < 0.0 {
        texture_u += 1.0;
    }

    let texture_x = texture_u * texture_size.x;

    let column_height = (screen_size.y * scale) / hit.dist;
    let column_y = screen_size.y * 0.5 - column_height * 0.5;
    let column_x = hit.index as f32;

    ColumnProjection {
        screen_pos: Vec2::new(column_x, column_y),
        screen_size: Vec2::new(1.0, column_height),
        tex_pos: Vec2::new(texture_x, 0.0),
        tex_size: Vec2::new(1.0, texture_size.y),
    }
}
