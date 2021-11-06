use crate::errors::VertexError;
use crate::traits::Graph;
use crate::types::{EID, VID};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};

/// Graph structure based on adjacency list storage.
pub struct AdjacencyListGraph {
    data: BTreeMap<VID, BTreeSet<VID>>,
}

impl PartialEq for AdjacencyListGraph {
    fn eq(&self, other: &Self) -> bool {
        // Compare maps.
        self.data == other.data
    }
}

impl Eq for AdjacencyListGraph {}

impl PartialOrd for AdjacencyListGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare maps.
        self.data.partial_cmp(&other.data)
    }
}

impl Graph for AdjacencyListGraph {
    fn new() -> Self {
        AdjacencyListGraph {
            // Initialize default empty map.
            data: BTreeMap::new(),
        }
    }

    fn from_order(order: usize) -> Self {
        let mut graph = Self::new();
        // Iterate over range inserting adjacency lists.
        graph.data.extend((0..order).map(|i| (i, BTreeSet::new())));
        graph
    }

    fn order(&self) -> usize {
        // Get map size.
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len() as usize)
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, v: &VID) -> bool {
        // Check if map contains key.
        self.data.contains_key(v)
    }

    fn add_vertex(&mut self, v: &VID) -> Result<(), VertexError> {
        // TODO: Update using try_insert once stable.
        if self.has_vertex(v) {
            return Err(VertexError);
        }
        match self.data.insert(*v, BTreeSet::new()) {
            Some(_) => Err(VertexError),
            None => Ok(()),
        }
    }

    fn del_vertex(&mut self, v: &VID) -> Result<(), VertexError> {
        // Remove vertex from map.
        match self.data.remove(v) {
            // If no vertex found return error.
            None => Err(VertexError),
            // Otherwise return successful.
            Some(_) => Ok(()),
        }
    }

    fn has_edge(&self, e: &EID) -> Result<bool, VertexError> {
        // Get vertex adjacency list.
        match self.data.get(&e.0) {
            // If no vertex found return error.
            None => Err(VertexError),
            // Otherwise check second vertex.
            Some(adj) => match self.data.contains_key(&e.1) {
                // If no vertex found return error.
                false => Err(VertexError),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(&e.1)),
            },
        }
    }

    fn add_edge(&mut self, e: &EID) -> Result<(), VertexError> {
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(VertexError),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(VertexError),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(e.1) {
                    // If no vertex inserted return error.
                    // FIXME: Change to EdgeError.
                    false => Err(VertexError),
                    // Otherwise return successful.
                    true => Ok(()),
                },
            },
        }
    }

    fn del_edge(&mut self, e: &EID) -> Result<(), VertexError> {
        // Check if second vertex exists.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(VertexError),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(VertexError),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.remove(&e.1) {
                    // If no vertex inserted return error.
                    // FIXME: Change to EdgeError.
                    false => Err(VertexError),
                    // Otherwise return successful.
                    true => Ok(()),
                },
            },
        }
    }
}

impl Debug for AdjacencyListGraph {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
