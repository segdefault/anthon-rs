use std::ops;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl Vec2F {
    pub fn dot(&self, other: &Vec2F) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(&self, other: &Vec2F) -> f32 {
        let x = (self.x - other.x).powf(2f32);
        let y = (self.y - other.y).powf(2f32);

        (y + x).sqrt()
    }

    pub fn angle(a: &Vec2F, b: &Vec2F, c: &Vec2F) -> f32 {
        let line_a = a - b;
        let line_b = c - b;

        let dot_product = line_a.dot(&line_b);
        let magnitude_a = line_a.dot(&line_a).sqrt();
        let magnitude_b = line_b.dot(&line_b).sqrt();

        let cos = (dot_product / magnitude_a / magnitude_b).acos();

        cos.to_degrees()
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
