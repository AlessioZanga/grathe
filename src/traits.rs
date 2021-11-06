use crate::types::{Result, EID, VID};
use std::fmt::Debug;

/// The base graph trait.
pub trait Graph: Eq + PartialOrd + Debug {
    /// Default constructor.
    fn new() -> Self;

    /// Construct the graph from a given order.
    fn from_order(order: usize) -> Self;

    /// Return the graph order (aka. the number of vertices).
    fn order(&self) -> usize;

    /// Return the graph size (aka. the number of edges).
    fn size(&self) -> usize;

    /// Checks whether the graph has a given vertex or not.
    fn has_vertex(&self, v: &VID) -> bool;

    /// Add given vertex to the graph.
    fn add_vertex(&mut self, v: &VID) -> Result<()>;

    /// Delete given vertex from the graph.
    fn del_vertex(&mut self, v: &VID) -> Result<()>;

    /// Checks whether the graph has a given edge or not.
    fn has_edge(&self, e: &EID) -> Result<bool>;

    /// Add given edge to the graph.
    fn add_edge(&mut self, e: &EID) -> Result<()>;

    /// Delete given edge from the graph.
    fn del_edge(&mut self, e: &EID) -> Result<()>;
}
