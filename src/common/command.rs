use std::process::Command as ProcessCommand;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumString, EnumVariantNames};
use tfc::MouseContext;

use crate::common::PointerTracker;

#[derive(Serialize, Deserialize, Display, EnumDiscriminants, EnumVariantNames)]
#[strum_discriminants(derive(Display, EnumString))]
pub enum Command {
    Disabled,
    Execute(String),
    Mouse(MouseButton, KeyEvent),
    Scroll(ScrollCommand),
}

#[derive(
    Serialize, Deserialize, Display, EnumVariantNames, EnumString, Clone, Copy, PartialEq, Eq,
)]
pub enum Axis {
    X,
    Y,
}

#[derive(Serialize, Deserialize)]
pub struct ScrollCommand {
    pub custom_command: Option<String>,
    pub factor: f32,
    pub axis: Axis,
}

impl Command {
    pub fn execute(&self, pointer: &mut PointerTracker) {
        match self {
            Command::Disabled => (),
            Command::Execute(cmd) => {
                ProcessCommand::new(cmd).spawn().ok();
            }
            Command::Mouse(button, event) => {
                match event {
                    KeyEvent::Press => pointer.context_mut().mouse_down(button.into()).ok(),
                    KeyEvent::Release => pointer.context_mut().mouse_up(button.into()).ok(),
                    KeyEvent::Click => pointer.context_mut().mouse_click(button.into()).ok(),
                };
            }
            Command::Scroll(cmd) => {
                let offset: i32 = (pointer.delta_y() * cmd.factor).round() as i32;
                if let Some(ref cmd) = cmd.custom_command {
                    ProcessCommand::new(cmd.to_owned() + offset.to_string().as_str())
                        .spawn()
                        .ok();
                } else {
                    match cmd.axis {
                        Axis::X => pointer.context_mut().mouse_scroll(offset, 0).ok(),
                        Axis::Y => pointer.context_mut().mouse_scroll(0, offset).ok(),
                    };
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Display, EnumVariantNames, EnumString)]
pub enum KeyEvent {
    Press,
    Release,
    Click,
}

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumVariantNames, EnumString)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<&MouseButton> for tfc::MouseButton {
    fn from(button: &MouseButton) -> Self {
        (*button).into()
    }
}

impl From<MouseButton> for tfc::MouseButton {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => tfc::MouseButton::Left,
            MouseButton::Right => tfc::MouseButton::Right,
            MouseButton::Middle => tfc::MouseButton::Middle,
        }
    }
}
