use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

pub use conditional_edge::ConditionalEdge;
pub use state_machine::StateMachine;

use crate::common::graph::Node;
use crate::common::{Axis, Command, ScrollCommand};

mod conditional_edge;
mod state_machine;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Display, EnumIter, EnumString)]
pub enum StateType {
    Basic,
    Pointing,
    Scrolling,
}

#[derive(Serialize, Deserialize, EnumString, Hash, PartialEq, Eq, Display)]
pub enum StateEvent {
    OnEnter,
    OnExit,
    OnScrollX,
    OnScrollY,
}

#[derive(Serialize, Deserialize)]
pub struct State<I: Eq + Hash> {
    index: I,
    pub name: String,

    // Less boilerplate code if events
    // are done this way rather than using
    // a more fabulous enum for the state type
    r#type: StateType,
    events: HashMap<StateEvent, Command>,

    // If you think that these should be stored somewhere else, meh, you are right.
    pub x: f32,
    pub y: f32,
}

impl<I: Eq + Hash> State<I> {
    pub fn new(index: I) -> State<I> {
        let mut events = HashMap::new();

        events.insert(StateEvent::OnEnter, Command::Disabled);
        events.insert(StateEvent::OnExit, Command::Disabled);

        State {
            index,
            name: String::from("New "),
            x: 0f32,
            y: 0f32,
            r#type: StateType::Basic,
            events,
        }
    }

    pub fn events(&self) -> &HashMap<StateEvent, Command> {
        &self.events
    }

    pub fn get_command(&self, event: &StateEvent) -> Option<&Command> {
        self.events.get(event)
    }

    // Does NOT insert new commands
    pub fn set_command(&mut self, event: StateEvent, command: Command) {
        if self.events.contains_key(&event) {
            self.events.insert(event, command);
        }
    }

    pub fn r#type(&self) -> StateType {
        self.r#type
    }

    // Modify the available events according to the selected states
    pub fn set_type(&mut self, r#type: StateType) {
        if self.r#type == r#type {
            return;
        }

        // Cleanup
        match self.r#type {
            StateType::Basic => (),
            StateType::Pointing => (),
            StateType::Scrolling => {
                self.events.remove(&StateEvent::OnScrollX);
                self.events.remove(&StateEvent::OnScrollY);
            }
        }

        // Augmentation
        self.r#type = r#type;
        match self.r#type {
            StateType::Basic => (),
            StateType::Pointing => (),
            StateType::Scrolling => {
                self.events.insert(
                    StateEvent::OnScrollX,
                    Command::Scroll(ScrollCommand {
                        custom_command: None,
                        factor: 1000f32,
                        axis: Axis::X,
                    }),
                );
                self.events.insert(
                    StateEvent::OnScrollY,
                    Command::Scroll(ScrollCommand {
                        custom_command: None,
                        factor: 1000f32,
                        axis: Axis::Y,
                    }),
                );
            }
        }
    }
}

impl<I: Eq + Hash + Copy> Node<I> for State<I> {
    fn id(&self) -> I {
        self.index
    }
}
