use crate::errors::VertexError;
use crate::traits::Graph;
use num::{FromPrimitive, Integer, ToPrimitive};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};

/// Graph structure based on adjacency list storage.
pub struct AdjacencyListGraph<T>
where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive,
{
    data: BTreeMap<T, BTreeSet<T>>,
}

impl<T> PartialEq for AdjacencyListGraph<T>
where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive,
{
    fn eq(&self, other: &Self) -> bool {
        // Compare maps.
        self.data == other.data
    }
}

impl<T> Eq for AdjacencyListGraph<T> where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive
{
}

impl<T> PartialOrd for AdjacencyListGraph<T>
where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare maps.
        self.data.partial_cmp(&other.data)
    }
}

impl<T> Graph for AdjacencyListGraph<T>
where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive,
{
    type VID = T;
    type EID = (T, T); // TODO: Remove once associated type defaults are stable.

    fn new() -> Self {
        AdjacencyListGraph {
            // Initialize default empty map.
            data: BTreeMap::new(),
        }
    }

    fn from<U: Integer + ToPrimitive>(order: U) -> Self {
        // Check for negative order.
        if order < U::zero() {
            panic!("Invalid negative order.");
        }
        // Initialize new graph.
        let mut graph = Self::new();
        // Iterate over range inserting adjacency lists.
        let mut i = U::zero();
        while i < order {
            graph.data.insert(
                Self::VID::from_usize(i.to_usize().unwrap()).unwrap(),
                BTreeSet::new(),
            );
            i = i + U::one();
        }
        graph
    }

    fn order(&self) -> usize {
        // Get map size.
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, v: &Self::VID) -> bool {
        // Check if map contains key.
        self.data.contains_key(v)
    }

    fn add_vertex(&mut self, v: &Self::VID) -> Result<(), VertexError> {
        // TODO: Update using try_insert once stable.
        if self.has_vertex(v) {
            return Err(VertexError);
        }
        match self.data.insert(*v, BTreeSet::new()) {
            Some(_) => Err(VertexError),
            None => Ok(()),
        }
    }

    fn del_vertex(&mut self, v: &Self::VID) -> Result<(), VertexError> {
        // Remove vertex from map.
        match self.data.remove(v) {
            // If no vertex found return error.
            None => Err(VertexError),
            // Otherwise return successful.
            Some(_) => Ok(()),
        }
    }

    fn has_edge(&self, e: &Self::EID) -> Result<bool, VertexError> {
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

    fn add_edge(&mut self, e: &Self::EID) -> Result<(), VertexError> {
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

    fn del_edge(&mut self, e: &Self::EID) -> Result<(), VertexError> {
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

impl<T> Debug for AdjacencyListGraph<T>
where
    T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
