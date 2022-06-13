use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::common::Edge;

#[derive(Serialize, Deserialize)]
pub struct ConditionalEdge<I, J> {
    next: I,
    pub trigger: J,
}

impl<I, J> ConditionalEdge<I, J> {
    pub fn new(next: I, trigger: J) -> Self {
        ConditionalEdge { next, trigger }
    }

    pub fn trigger(&self) -> &J {
        &self.trigger
    }
}

impl<I: Eq + Hash + Copy, J: Eq + Hash> Edge<I> for ConditionalEdge<I, J> {
    fn next(&self) -> I {
        self.next
    }
}
