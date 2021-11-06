use crate::errors::VertexError;
use num::{FromPrimitive, Integer, ToPrimitive};
use std::fmt::Debug;

/// The base graph trait.
pub trait Graph: Eq + PartialOrd + Debug {
    /// Vertex identifier type.
    // TODO: Change FromPrimitive to Step once stable,
    // use forward(1) to increase VID in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type VID: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive;

    /// Edge identifier type.
    // TODO: Change to EID = (VID, VID) once associated type defaults are stable.
    type EID: Sized + Eq + Ord + Copy + Debug + Default;

    /// Default constructor.
    fn new() -> Self;

    /// Construct the graph from a given order.
    fn from<U: Integer + ToPrimitive>(order: U) -> Self;

    /// Return the graph order (aka. the number of vertices).
    fn order(&self) -> usize;

    /// Return the graph size (aka. the number of edges).
    fn size(&self) -> usize;

    /// Checks whether the graph has a given vertex or not.
    fn has_vertex(&self, v: &Self::VID) -> bool;

    /// Add given vertex to the graph.
    fn add_vertex(&mut self, v: &Self::VID) -> Result<(), VertexError>;

    /// Delete given vertex from the graph.
    fn del_vertex(&mut self, v: &Self::VID) -> Result<(), VertexError>;

    /// Checks whether the graph has a given edge or not.
    fn has_edge(&self, e: &Self::EID) -> Result<bool, VertexError>;

    /// Add given edge to the graph.
    fn add_edge(&mut self, e: &Self::EID) -> Result<(), VertexError>;

    /// Delete given edge from the graph.
    fn del_edge(&mut self, e: &Self::EID) -> Result<(), VertexError>;
}
