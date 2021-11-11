use crate::errors::VertexError;
use num::FromPrimitive;
use std::fmt::Debug;

/// The base vertex trait.
pub trait VertexTrait: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive {}

// Blanket implementation of the vertex trait.
impl<T> VertexTrait for T where T: Sized + Eq + Ord + Copy + Debug + Default + FromPrimitive {}

/// The base edge trait.
pub trait EdgeTrait: Sized + Eq + Ord + Copy + Debug + Default {}

// Blanket implementation of the edge trait.
impl<T> EdgeTrait for T where T: Sized + Eq + Ord + Copy + Debug + Default {}

/// The base graph trait.
pub trait GraphTrait: Eq + PartialOrd + Debug + From<usize> {
    /// Vertex identifier type.
    // TODO: Change FromPrimitive to Step once stable, use combination of v = T::default()
    // and v = Step::forward(v, 1) to increase Vertex in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type Vertex: VertexTrait;

    /// Edge identifier type.
    // TODO: Change to Edge = (Vertex, Vertex) once associated type defaults are stable.
    type Edge: EdgeTrait;

    /// Storage type.
    type Storage;

    /// Default constructor.
    fn new() -> Self;

    /// Vertex iterator.
    fn v_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Vertex> + 'a>;

    /// Return immutable reference to data storage.
    fn data(&self) -> &Self::Storage;

    /// Return the graph order (aka. the number of vertices).
    fn order(&self) -> usize;

    /// Return the graph size (aka. the number of edges).
    fn size(&self) -> usize;

    /// Checks whether the graph has a given vertex or not.
    fn has_vertex(&self, v: &Self::Vertex) -> bool;

    /// Add given vertex to the graph.
    fn add_vertex(&mut self, v: &Self::Vertex) -> Result<(), VertexError>;

    /// Delete given vertex from the graph.
    fn del_vertex(&mut self, v: &Self::Vertex) -> Result<(), VertexError>;

    /// Checks whether the graph has a given edge or not.
    fn has_edge(&self, e: &Self::Edge) -> Result<bool, VertexError>;

    /// Add given edge to the graph.
    fn add_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;

    /// Delete given edge from the graph.
    fn del_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;
}
