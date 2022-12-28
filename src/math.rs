#[derive(Debug, Copy, Clone)]
pub struct Vec2I32 {
    pub x: i32,
    pub y: i32
}

pub fn vec2i32(x: i32, y: i32) -> Vec2I32 {
    Vec2I32 {
        x,
        y
    }
}