use crate::traits::Graph;
use crate::types::{Result, VertexError, EID, VID};
use std::collections::{BTreeMap, BTreeSet};

/// Graph structure based on adjacency list storage.
struct AdjacencyListGraph {
    data: BTreeMap<VID, BTreeSet<VID>>,
}

impl Graph for AdjacencyListGraph {
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
        match self.data.insert(*v, BTreeSet::<VID>::new()) {
            Some(_) => Ok(()),
            None => Err(Box::new(VertexError)),
        }
    }

    fn del_vertex(&mut self, v: &VID) -> Result<()> {
        match self.data.remove(v) {
            Some(_) => Ok(()),
            None => Err(Box::new(VertexError)),
        }
    }

    fn has_edge(&self, e: &EID) -> Result<bool> {
        if !(self.has_vertex(&e.0) && self.has_vertex(&e.1)) {
            return Err(Box::new(VertexError));
        }
        Ok(self.data.get(&e.0).map(|adj| adj.contains(&e.1)).is_none())
    }

    fn add_edge(&mut self, e: &EID) -> Result<()> {
        let has_edge = self.has_edge(e);
        let has_edge = has_edge.is_err() || has_edge.unwrap();
        if has_edge {
            return Err(Box::new(VertexError));
        }
        self.data.get_mut(&e.0).unwrap().insert(e.1);
        Ok(())
    }

    fn del_edge(&mut self, e: &EID) -> Result<()> {
        let has_edge = self.has_edge(e);
        let has_edge = has_edge.is_err() || has_edge.unwrap();
        if !has_edge {
            return Err(Box::new(VertexError));
        }
        self.data.get_mut(&e.0).unwrap().remove(&e.1);
        Ok(())
    }
}
