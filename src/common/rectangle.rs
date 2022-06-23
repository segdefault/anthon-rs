use std::fmt::Debug;

use imageproc::rect::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn end_x(&self) -> f32 {
        self.x + self.width
    }

    pub fn end_y(&self) -> f32 {
        self.y + self.height
    }

    // Move the rectangle to contain the point
    pub fn contain(&mut self, x: f32, y: f32) {
        if x < self.x {
            self.x = self.x - (self.x - x);
        } else if x > self.end_x() {
            self.x += x - self.end_x();
        }

        if y < self.y {
            self.y = self.y - (self.y - y);
        } else if y > self.end_y() {
            self.y += y - self.end_y();
        }
    }

    pub fn bound(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        self.width = self.width.min(max_x - min_x);
        self.height = self.height.min(max_y - min_y);

        self.x = min_x.max((max_x - self.width).min(self.x));
        self.y = min_y.max((max_y - self.height).min(self.y));
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn scale(&mut self, center_x: f32, center_y: f32, factor: f32) {
        // self.x = self.x * factor + center_x * factor;
        // self.y = self.y * factor + center_y * factor;

        self.translate(-center_x, -center_y);
        self.x *= factor;
        self.y *= factor;
        self.translate(center_x, center_y);

        self.width *= factor;
        self.height *= factor;
    }

    pub fn multiply(&mut self, x_factor: f32, y_factor: f32) {
        self.x *= x_factor;
        self.y *= y_factor;

        self.width *= x_factor;
        self.height *= y_factor;
    }
}

impl From<Rectangle> for Rect {
    fn from(rect: Rectangle) -> Self {
        Rect::at(rect.x as i32, rect.y as i32).of_size(rect.width as u32, rect.height as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Rectangle;

    #[test]
    fn scale_begin() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 1f32,
            height: 1f32,
        };
        let expected_rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 0.5f32,
            height: 0.5f32,
        };

        rect.scale(0f32, 0f32, 0.5);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn scale_middle() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 1f32,
            height: 1f32,
        };
        let expected_rect = Rectangle {
            x: 0.25f32,
            y: 0.25f32,
            width: 0.5f32,
            height: 0.5f32,
        };

        rect.scale(0.5f32, 0.5f32, 0.5);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn scale_end() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 1f32,
            height: 1f32,
        };
        let expected_rect = Rectangle {
            x: 0.5f32,
            y: 0.5f32,
            width: 0.5f32,
            height: 0.5f32,
        };

        rect.scale(1f32, 1f32, 0.5);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn scale_middle_2() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 1f32,
            height: 1f32,
        };
        let expected_rect = Rectangle {
            x: 0.4f32,
            y: 0.4f32,
            width: 0.2f32,
            height: 0.2f32,
        };

        rect.scale(0.5f32, 0.5f32, 0.2);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn scale_random() {
        let mut rect = Rectangle {
            x: 2f32,
            y: 3f32,
            width: 4f32,
            height: 6f32,
        };
        let expected_rect = Rectangle {
            x: 4f32,
            y: 3f32,
            width: 2f32,
            height: 3f32,
        };

        rect.scale(6f32, 3f32, 0.5);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn contain_positive_positive() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 50f32,
            height: 50f32,
        };
        let expected_rect = Rectangle {
            x: 10f32,
            y: 10f32,
            width: 50f32,
            height: 50f32,
        };

        rect.contain(60f32, 60f32);

        assert_eq!(rect, expected_rect);
    }

    #[test]
    fn contain_negative_negative() {
        let mut rect = Rectangle {
            x: 0f32,
            y: 0f32,
            width: 50f32,
            height: 50f32,
        };
        let expected_rect = Rectangle {
            x: -10f32,
            y: -10f32,
            width: 50f32,
            height: 50f32,
        };

        rect.contain(-10f32, -10f32);

        assert_eq!(rect, expected_rect);
    }
}
