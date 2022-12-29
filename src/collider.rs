use crate::math::Vec2I32;

/// Collider with x, y, width, and height that can check for AABB Collision Overlap with any other collider.
/// Can Be Disabled
#[derive(Clone, Debug, PartialEq)]
pub struct Collider {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub enabled: bool
}

impl Collider {
    /// Creates a collider with enabled true
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            enabled: true
        }
    }

    pub fn min_x(&self) -> i32 {
        self.x
    }
    pub fn min_y(&self) -> i32 {
        self.y
    }
    pub fn max_x(&self) -> i32 {
        self.x + self.width
    }
    pub fn max_y(&self) -> i32 {
        self.y + self.height
    }

    pub fn center_x(&self) -> i32 {
        self.x + self.width / 2
    }
    pub fn center_y(&self) -> i32 {
        self.y + self.height / 2
    }

    /// Checks for AABB Collision between collider and self with an offset
    pub fn is_overlapping(&self, offset: Vec2I32, other: &Collider) -> bool {
        if !(self.enabled && other.enabled) { return false };
        (self.max_x() + offset.x) > other.min_x()
            && (self.min_x() + offset.x) < other.max_x()
            && (self.max_y() + offset.y) > other.min_y()
            && (self.min_y() + offset.y) < other.max_y()
    }
}