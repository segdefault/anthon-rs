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

struct ProcessCommandSplitter {
    command: String,
}

impl From<String> for ProcessCommandSplitter {
    fn from(command: String) -> Self {
        Self { command }
    }
}

impl From<ProcessCommandSplitter> for String {
    fn from(command: ProcessCommandSplitter) -> Self {
        command.command
    }
}

impl From<ProcessCommandSplitter> for ProcessCommand {
    fn from(command: ProcessCommandSplitter) -> Self {
        let command = String::from(command);
        let mut args = command.trim().split(' ');
        let mut command = ProcessCommand::new(args.next().unwrap());

        for arg in args {
            command.arg(arg);
        }

        command
    }
}

impl Command {
    pub fn execute(&self, pointer: &mut PointerTracker) {
        match self {
            Command::Disabled => (),
            Command::Execute(cmd) => {
                ProcessCommand::from(ProcessCommandSplitter::from(cmd.clone()))
                    .spawn()
                    .ok();
            }
            Command::Mouse(button, event) => {
                match event {
                    KeyEvent::Press => pointer.context_mut().mouse_down(button.into()).ok(),
                    KeyEvent::Release => pointer.context_mut().mouse_up(button.into()).ok(),
                    KeyEvent::Click => pointer.context_mut().mouse_click(button.into()).ok(),
                };
            }
            Command::Scroll(cmd) => {
                let offset_x: i32 = (pointer.delta_x() * cmd.factor).round() as i32;
                let offset_y: i32 = (pointer.delta_y() * cmd.factor).round() as i32;

                if let Some(ref custom_cmd) = cmd.custom_command {
                    ProcessCommand::from(ProcessCommandSplitter::from(custom_cmd.clone()))
                        .arg(
                            match cmd.axis {
                                Axis::X => offset_x,
                                Axis::Y => offset_y,
                            }
                            .to_string(),
                        )
                        .spawn()
                        .ok();
                } else {
                    match cmd.axis {
                        Axis::X => pointer.context_mut().mouse_scroll(offset_x, 0).ok(),
                        Axis::Y => pointer.context_mut().mouse_scroll(0, offset_y).ok(),
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
