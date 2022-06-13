use std::fmt::Display;
use std::hash::Hash;
use std::{thread, time};

use crate::common::state::StateEvent;
use crate::common::{Edge, Graph, PointerTracker};

use super::{ConditionalEdge, State};

pub struct StateMachine<I: Eq + Hash> {
    current_state: I,
}

impl<I: Eq + Hash + Copy> StateMachine<I> {
    pub fn new(current_state: I) -> Self {
        let machine = StateMachine { current_state };
        thread::sleep(time::Duration::from_millis(10)); // Needed to create a new context

        machine
    }

    pub fn current_state(&self) -> &I {
        &self.current_state
    }

    pub fn process<J: Eq + Hash + Display>(
        &mut self,
        state_graph: &Graph<I, State<I>, ConditionalEdge<I, Option<J>>>,
        trigger: &J,
        pointer: &mut PointerTracker,
    ) -> bool {
        let edges = state_graph
            .edges()
            .get(&self.current_state)
            .expect("ERROR: Invalid state ID");

        for edge in edges.values() {
            if let Some(edge_trigger) = edge.trigger() {
                if edge_trigger == trigger {
                    self.try_execute(&StateEvent::OnExit, state_graph, pointer);
                    self.current_state = edge.next();
                    self.try_execute(&StateEvent::OnEnter, state_graph, pointer);

                    return true;
                }
            }
        }

        false
    }

    fn try_execute<J: Eq + Hash + Display>(
        &mut self,
        event: &StateEvent,
        state_graph: &Graph<I, State<I>, ConditionalEdge<I, Option<J>>>,
        pointer: &mut PointerTracker,
    ) {
        if let Some(cmd) = state_graph
            .get_node(&self.current_state)
            .expect("ERROR: Invalid current state ID.")
            .get_command(event)
        {
            cmd.execute(pointer);
        }
    }
}
