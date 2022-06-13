use std::{thread, time::Duration};

use tfc::{Context, Error, MouseContext, ScreenContext};

use crate::common::filter::Wmaf32;
use crate::common::{Point2F, Rectangle};
use crate::mediapipe::Packet;

// TODO: Move these to the config file
const PRECISION_FACTOR: f32 = 2f32;
pub const DEFAULT_WMA_ORDER: usize = 5;

pub struct PointerTracker {
    pub freeze: bool,

    // Virtual Coordinates according to the virtual rectangle
    // Range: [0, 1]
    x: Wmaf32,
    y: Wmaf32,
    delta_x: f32,
    delta_y: f32,

    virtual_screen: Rectangle,
    context: Context,
}

impl PointerTracker {
    pub fn new(wma_order: usize) -> Result<Self, Error> {
        let context = Context::new()?;
        // For OS-specific reasons, it's necessary to wait a moment after
        // creating the context before generating events.
        thread::sleep(Duration::from_millis(10));

        let cursor = context.cursor_location()?;
        let screen = context.screen_size()?;

        let v_size = 1f32;
        let x = cursor.0 as f32 / screen.0 as f32;
        let y = cursor.1 as f32 / screen.1 as f32;

        let mut virtual_screen = Rectangle {
            x: 0f32,
            y: 0f32,
            width: v_size,
            height: v_size,
        };

        virtual_screen.bound(0f32, 0f32, 1f32, 1f32);
        let tracker = Self {
            freeze: true,

            x: Wmaf32::new_from(wma_order, x),
            y: Wmaf32::new_from(wma_order, y),
            delta_x: 0f32,
            delta_y: 0f32,

            virtual_screen,

            context: Context::new()?,
        };

        Ok(tracker)
    }

    pub fn x(&self) -> f32 {
        *self.x
    }
    pub fn y(&self) -> f32 {
        *self.y
    }

    pub fn delta_x(&self) -> f32 {
        self.delta_x
    }

    pub fn delta_y(&self) -> f32 {
        self.delta_y
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn track(&mut self, packet: &Packet) -> Result<(), Error> {
        if let Some(ref landmarks) = packet.landmarks {
            let prev_x = *self.x;
            let prev_y = *self.y;
            let new_x = (landmarks[5].x + landmarks[17].x).abs() / 2f32;
            let new_y = (landmarks[5].y + landmarks[17].y).abs() / 2f32;
            let prev_virtual_x = (prev_x - self.virtual_screen.x) / self.virtual_screen.width;
            let prev_virtual_y = (prev_y - self.virtual_screen.y) / self.virtual_screen.height;

            let delta_x = smooth(new_x - prev_x, 0.5f32);
            let delta_y = smooth(new_y - prev_y, 0.5f32);
            self.x.set_value(prev_x + delta_x);
            self.y.set_value(prev_y + delta_y);

            self.update_virtual_screen(landmarks)?;
            let virtual_x = (*self.x - self.virtual_screen.x) / self.virtual_screen.width;
            let virtual_y = (*self.y - self.virtual_screen.y) / self.virtual_screen.height;

            self.delta_x = virtual_x - prev_virtual_x;
            self.delta_y = virtual_y - prev_virtual_y;

            if !self.freeze {
                let screen = self.context.screen_size()?;

                let x_ratio =
                    min_max_normal((*self.x - self.virtual_screen.x) / self.virtual_screen.width);
                let y_ratio =
                    min_max_normal((*self.y - self.virtual_screen.y) / self.virtual_screen.height);

                let real_x = x_ratio * screen.0 as f32;
                let real_y = y_ratio * screen.1 as f32;

                self.context.mouse_move_abs(real_x as i32, real_y as i32)?;
            }
        }

        Ok(())
    }

    fn update_virtual_screen(&mut self, landmarks: &[Point2F]) -> Result<(), Error> {
        let v_size = ((landmarks[5].x - landmarks[17].x).abs() * PRECISION_FACTOR).min(1f32);

        self.virtual_screen
            .scale(*self.x, *self.y, v_size / self.virtual_screen.width);
        self.virtual_screen.contain(*self.x, *self.y);
        self.virtual_screen.bound(0f32, 0f32, 1f32, 1f32);

        Ok(())
    }
}

fn min_max_normal(value: f32) -> f32 {
    0f32.max(1f32.min(value))
}

pub fn smooth(value: f32, epsilon: f32) -> f32 {
    let mut value_normal = min_max_normal(value.abs());
    value_normal = 2f32.powf(value_normal.powf(2f32 - epsilon)) - 1f32;

    value_normal.copysign(value)
}
