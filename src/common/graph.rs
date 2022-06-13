use std::collections::hash_map::IterMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use serde::{Deserialize, Serialize};

pub trait Edge<I: Eq + Hash> {
    fn next(&self) -> I;
}

pub trait Node<I: Eq + Hash> {
    fn id(&self) -> I;
}

#[derive(Serialize, Deserialize)]
pub struct Graph<I: Eq + Hash + Copy, N: Node<I>, E: Edge<I>> {
    nodes: HashMap<I, N>,
    edges: HashMap<I, HashMap<I, E>>,
    // At whom node[I] points
    inverse_edges: HashMap<I, HashSet<I>>, // Who points at node[I]
}

impl<I: Eq + Hash + Copy, N: Node<I>, E: Edge<I>> Graph<I, N, E> {
    pub fn nodes(&self) -> &HashMap<I, N> {
        &self.nodes
    }

    pub fn edges(&self) -> &HashMap<I, HashMap<I, E>> {
        &self.edges
    }

    pub fn edge_iter_mut(&mut self) -> IterMut<'_, I, HashMap<I, E>> {
        self.edges.iter_mut()
    }

    pub fn get_node(&self, index: &I) -> Option<&N> {
        self.nodes.get(index)
    }

    pub fn get_node_mut(&mut self, index: &I) -> Option<&mut N> {
        self.nodes.get_mut(index)
    }

    pub fn add_node(&mut self, node: N) -> bool {
        let node_id = node.id();

        if let std::collections::hash_map::Entry::Vacant(e) = self.nodes.entry(node_id) {
            e.insert(node);
            self.edges.insert(node_id, HashMap::new());
            self.inverse_edges.insert(node_id, HashSet::new());
            true
        } else {
            false
        }
    }

    pub fn delete_node(&mut self, id: &I) -> Option<N> {
        // Delete the edges that point at this node
        if let Some(neighbours) = self.inverse_edges.remove(id) {
            for neighbour_id in neighbours {
                self.edges
                    .get_mut(&neighbour_id)
                    .expect("ERROR: Inconsistent graph.")
                    .remove(id);
            }
        }

        // Delete the inverse edges that point at this node
        if let Some(edges) = self.edges.remove(id) {
            for edge in edges {
                let neighbour_id = edge.0;
                self.inverse_edges
                    .get_mut(&neighbour_id)
                    .expect("ERROR: Inconsistent graph.")
                    .remove(id);
            }
        }

        self.nodes.remove(id)
    }

    pub fn get_edge(&self, from_id: &I, to_id: &I) -> Option<&E> {
        self.edges.get(from_id)?.get(to_id)
    }

    pub fn get_edge_mut(&mut self, from_id: &I, to_id: &I) -> Option<&mut E> {
        self.edges.get_mut(from_id)?.get_mut(to_id)
    }

    // FIXME: What the hell is this
    pub fn add_edge(&mut self, from_id: &I, edge: E) -> bool {
        let to_id = edge.next();

        if let Some(edges) = self.edges.get_mut(from_id) {
            edges.insert(to_id, edge);
        }

        if let Some(inverse_edges) = self.inverse_edges.get_mut(&to_id) {
            inverse_edges.insert(*from_id);
            return true;
        }

        false
    }

    pub fn delete_edge(&mut self, from_id: &I, to_id: &I) {
        self.edges
            .get_mut(from_id)
            .expect("ERROR: Invalid from node ID")
            .remove(to_id)
            .expect("ERROR: Invalid to node ID");
        self.inverse_edges
            .get_mut(to_id)
            .expect("ERROR: Invalid to node ID")
            .remove(from_id);
    }
}

impl<I: Eq + Hash + Copy, N: Node<I>, E: Edge<I>> Default for Graph<I, N, E> {
    fn default() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            inverse_edges: HashMap::new(),
        }
    }
}
