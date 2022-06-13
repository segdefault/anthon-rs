use std::ops;

use crate::common::Point2F;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl Vec2F {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vec2F) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle(&self, other: &Vec2F) -> f32 {
        let dot = self.dot(other);

        (dot / self.magnitude() / other.magnitude()).acos()
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }
}

impl Default for Vec2F {
    fn default() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl ops::Add<&Vec2F> for &Vec2F {
    type Output = Vec2F;

    fn add(self, other: &Vec2F) -> <Self as std::ops::Sub<&Vec2F>>::Output {
        Vec2F {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<&Vec2F> for &Vec2F {
    type Output = Vec2F;

    fn sub(self, other: &Vec2F) -> <Self as std::ops::Sub<&Vec2F>>::Output {
        Vec2F {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<Point2F> for Vec2F {
    fn from(p: Point2F) -> Self {
        Vec2F { x: p.x, y: p.y }
    }
}

impl From<Vec2F> for Point2F {
    fn from(v: Vec2F) -> Self {
        Point2F { x: v.x, y: v.y }
    }
}

impl From<(Point2F, Point2F)> for Vec2F {
    fn from(v: (Point2F, Point2F)) -> Self {
        Vec2F {
            x: v.1.x - v.0.x,
            y: v.1.y - v.0.y,
        }
    }
}
