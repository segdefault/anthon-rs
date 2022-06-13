use crate::common::Vec2F;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Point2F {
    pub x: f32,
    pub y: f32,
}

impl Default for Point2F {
    fn default() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl Point2F {
    pub fn new(x: f32, y: f32) -> Point2F {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        Vec2F::from((*self, *other)).magnitude()
    }
}
