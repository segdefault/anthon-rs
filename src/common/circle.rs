use crate::common::Vec2F;

pub struct Circle {
    center: Vec2F,
    radius: f32,
}

impl Circle {
    pub fn new(points: &[Vec2F]) -> Self {
        let mut center = points
            .iter()
            .fold(Vec2F { x: 0f32, y: 0f32 }, |mut acc, p| {
                acc.x += p.x;
                acc.y += p.y;

                acc
            });

        center.x /= points.len() as f32;
        center.y /= points.len() as f32;

        let radius = points[0].distance(&center);

        Circle { center, radius }
    }

    pub fn contains(&self, point: &Vec2F) -> bool {
        self.center.distance(point) < self.radius
    }
}
