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
pub trait GraphTrait: Eq + PartialOrd + Debug + Default + From<usize> {
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
    fn v_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a>;

    /// Edge iterator.
    fn e_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Edge> + 'a>;

    /// Return immutable reference to data storage.
    fn data(&self) -> &Self::Storage;

    /// Return the graph order (aka. the number of vertices).
    fn order(&self) -> usize;

    /// Return the graph size (aka. the number of edges).
    fn size(&self) -> usize;

    /// Checks whether the graph has a given vertex or not.
    fn has_vertex(&self, v: &Self::Vertex) -> bool;

    /// Add given vertex to the graph, panic on invalid argument.
    fn add_vertex(&mut self, v: &Self::Vertex) -> () {
        return self.try_add_vertex(v).unwrap();
    }

    /// Delete given vertex from the graph, panic on invalid argument.
    fn del_vertex(&mut self, v: &Self::Vertex) -> () {
        return self.try_del_vertex(v).unwrap();
    }

    /// Checks whether the graph has a given edge or not, panic on invalid argument.
    fn has_edge(&self, e: &Self::Edge) -> bool {
        return self.try_has_edge(e).unwrap();
    }

    /// Add given edge to the graph, panic on invalid argument.
    fn add_edge(&mut self, e: &Self::Edge) -> () {
        return self.try_add_edge(e).unwrap();
    }

    /// Delete given edge from the graph, panic on invalid argument.
    fn del_edge(&mut self, e: &Self::Edge) -> () {
        return self.try_del_edge(e).unwrap();
    }

    /// Add given vertex to the graph, return error on invalid argument.
    fn try_add_vertex(&mut self, v: &Self::Vertex) -> Result<(), VertexError>;

    /// Delete given vertex from the graph, return error on invalid argument.
    fn try_del_vertex(&mut self, v: &Self::Vertex) -> Result<(), VertexError>;

    /// Checks whether the graph has a given edge or not, return error on invalid argument.
    fn try_has_edge(&self, e: &Self::Edge) -> Result<bool, VertexError>;

    /// Add given edge to the graph, return error on invalid argument.
    fn try_add_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;

    /// Delete given edge from the graph, return error on invalid argument.
    fn try_del_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;
}

/// Return the vertex iterator.
#[macro_export]
macro_rules! V {
    ($x:expr) => {
        $x.v_iter()
    };
}

/// Return the edge iterator.
#[macro_export]
macro_rules! E {
    ($x:expr) => {
        $x.e_iter()
    };
}
