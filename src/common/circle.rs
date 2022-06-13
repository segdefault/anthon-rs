use crate::common::Point2F;

pub struct Circle {
    center: Point2F,
    radius: f32,
}

impl From<&[Point2F]> for Circle {
    fn from(points: &[Point2F]) -> Self {
        let mut center = points.iter().fold(Point2F::default(), |mut acc, p| {
            acc.x += p.x;
            acc.y += p.y;

            acc
        });

        center.x /= points.len() as f32;
        center.y /= points.len() as f32;

        let radius = points
            .iter()
            .fold(0f32, |acc, p| acc.max(p.distance(&center)));

        Circle { center, radius }
    }
}

impl Circle {
    pub fn contains(&self, point: &Point2F) -> bool {
        self.center.distance(point) < self.radius
    }
}
