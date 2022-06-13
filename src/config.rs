use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::common::state::State;
use crate::common::SignDictionary;
use crate::{ConditionalGraph, StateIndex};

// pub const INITIAL_STATE_INDEX: StateIndex = StateIndex::MIN;
pub const INITIAL_STATE_INDEX: StateIndex = 0;
pub const INITIAL_STATE_NAME: &str = "Start";

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    YamlError(serde_yaml::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Error::IOError(e) => e.to_string(),
                Error::YamlError(e) => e.to_string(),
            }
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlError(e)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    sign_dictionary: SignDictionary,
    state_graph: ConditionalGraph,
    last_node_id: i32,

    pub sign_switching_smoothness: f32,
    pub sign_probability_threshold: f32,
}

impl Config {
    pub fn initial_state(&self) -> &State<StateIndex> {
        self.state_graph
            .nodes()
            .get(&INITIAL_STATE_INDEX)
            .expect("Initial state does not exist")
    }

    pub fn sign_dictionary(&self) -> &SignDictionary {
        &self.sign_dictionary
    }

    pub fn sign_dictionary_mut(&mut self) -> &mut SignDictionary {
        &mut self.sign_dictionary
    }

    pub fn state_graph(&self) -> &ConditionalGraph {
        &self.state_graph
    }

    pub fn state_graph_mut(&mut self) -> &mut ConditionalGraph {
        &mut self.state_graph
    }

    fn next_node_id(&mut self) -> StateIndex {
        let mut id = self.last_node_id;

        while {
            if id == StateIndex::MAX {
                id = StateIndex::MIN;
            } else {
                id += 1;
            }

            if id == self.last_node_id {
                panic!("ERROR: Maximum nodes reached.");
            }

            self.state_graph().nodes().contains_key(&id)
        } {}

        self.last_node_id = id;
        id
    }

    pub fn new_state(&mut self) -> &mut State<StateIndex> {
        let id = self.next_node_id();
        let mut state = State::new(id);
        state.name = format!("{}", id);
        self.state_graph.add_node(state);

        self.state_graph
            .get_node_mut(&id)
            .expect("ERROR: Invalid next node ID.")
    }

    pub fn from_file(path: &str) -> Result<Config, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(serde_yaml::from_str(contents.as_str())?)
    }

    pub fn save(&self, path: &str) -> Result<(), Error> {
        let mut file = File::create(path)?;
        file.write_all(serde_yaml::to_string(self)?.as_bytes())?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut state_graph = ConditionalGraph::default();
        let mut initial_state = State::new(INITIAL_STATE_INDEX);
        initial_state.name = String::from(INITIAL_STATE_NAME);

        state_graph.add_node(initial_state);

        Config {
            sign_dictionary: SignDictionary::from(BTreeMap::new()),
            state_graph,
            last_node_id: INITIAL_STATE_INDEX,

            sign_switching_smoothness: 1f32,
            sign_probability_threshold: 0.9f32,
        }
    }
}
