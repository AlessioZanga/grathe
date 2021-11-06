use crate::traits::Graph;
use crate::types::{Result, VertexError, EID, VID};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};

/// Graph structure based on adjacency list storage.
pub struct AdjacencyListGraph {
    data: BTreeMap<VID, BTreeSet<VID>>,
}

impl PartialEq for AdjacencyListGraph {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for AdjacencyListGraph {}

impl PartialOrd for AdjacencyListGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Graph for AdjacencyListGraph {
    fn new() -> Self {
        AdjacencyListGraph {
            data: BTreeMap::new(),
        }
    }

    fn from(order: usize) -> Self {
        let mut graph = Self::new();
        for v in 0..order {
            graph.add_vertex(&v).unwrap();
        }
        graph
    }

    fn order(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter()
            .map(|(_, adj)| adj.len() as usize)
            .sum::<usize>()
    }

    fn has_vertex(&self, v: &VID) -> bool {
        self.data.contains_key(v)
    }

    fn add_vertex(&mut self, v: &VID) -> Result<()> {
        if self.has_vertex(v) {
            return Err(Box::new(VertexError));
        }
        match self.data.insert(*v, BTreeSet::<VID>::new()) {
            Some(_) => Err(Box::new(VertexError)),
            None => Ok(()),
        }
    }

    fn del_vertex(&mut self, v: &VID) -> Result<()> {
        if !self.has_vertex(v) {
            return Err(Box::new(VertexError));
        }
        match self.data.remove(v) {
            Some(_) => Ok(()),
            None => Err(Box::new(VertexError)),
        }
    }

    fn has_edge(&self, e: &EID) -> Result<bool> {
        if !(self.has_vertex(&e.0) && self.has_vertex(&e.1)) {
            return Err(Box::new(VertexError));
        }
        Ok(self.data.get(&e.0).map(|adj| adj.contains(&e.1)).unwrap())
    }

    fn add_edge(&mut self, e: &EID) -> Result<()> {
        if matches!(self.has_edge(e), Ok(true) | Err(_)) {
            return Err(Box::new(VertexError));
        }
        self.data.get_mut(&e.0).unwrap().insert(e.1);
        Ok(())
    }

    fn del_edge(&mut self, e: &EID) -> Result<()> {
        if matches!(self.has_edge(e), Ok(false) | Err(_)) {
            return Err(Box::new(VertexError));
        }
        self.data.get_mut(&e.0).unwrap().remove(&e.1);
        Ok(())
    }
}

impl Debug for AdjacencyListGraph {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
