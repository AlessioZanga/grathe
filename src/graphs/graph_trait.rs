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
    // TODO: Change FromPrimitive to Step once stable, use combination of x = T::default()
    // and x = Step::forward(x, 1) to increase Vertex in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type Vertex: VertexTrait;

    /// Edge identifier type.
    // TODO: Change to Edge = (Vertex, Vertex) once associated type defaults are stable.
    type Edge: EdgeTrait;

    /// Storage type.
    type Storage;

    /// Base constructor.
    ///
    /// Returns a null graph.
    ///
    fn new() -> Self;

    /// Vertex iterator.
    /// 
    /// Iterates over the vertex set $V$ ordered by identifier value.
    /// 
    fn vertices_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a>;

    /// Edge iterator.
    /// 
    /// Iterates over the edge set $E$ order by identifier values.
    /// 
    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Edge> + 'a>;

    /// Data storage.
    /// 
    /// Return immutable reference to internal data storage.
    /// 
    fn data(&self) -> &Self::Storage;

    /// Order of the graph.
    /// 
    /// Return the graph order (aka. $|V|$).
    /// 
    fn order(&self) -> usize;

    /// Size of the graph.
    /// 
    /// Return the graph size (aka. $|E|$).
    /// 
    fn size(&self) -> usize;

    /// Checks vertex in the graph.
    ///
    /// Checks whether the graph has a given vertex or not.
    ///
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds vertex to the graph
    ///
    /// Insert given vertex identifier into the graph.
    ///
    /// # Panics
    ///
    /// The vertex identifier already exists in the graph.
    ///
    fn add_vertex(&mut self, x: &Self::Vertex) -> () {
        return self.try_add_vertex(x).unwrap();
    }

    /// Deletes vertex from the graph
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Panics
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> () {
        return self.try_del_vertex(x).unwrap();
    }

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    fn has_edge(&self, e: &Self::Edge) -> bool {
        return self.try_has_edge(e).unwrap();
    }

    /// Adds edge to the graph.
    ///
    /// Insert given edge identifier into the graph.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    fn add_edge(&mut self, e: &Self::Edge) -> () {
        return self.try_add_edge(e).unwrap();
    }

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier does not exists in the graph.
    ///
    fn del_edge(&mut self, e: &Self::Edge) -> () {
        return self.try_del_edge(e).unwrap();
    }

    /// Adds vertex to the graph
    ///
    /// Insert given vertex identifier into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier already exists in the graph.
    ///
    fn try_add_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError>;

    /// Deletes vertex from the graph
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    fn try_del_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    fn try_has_edge(&self, e: &Self::Edge) -> Result<bool, VertexError>;

    /// Adds edge to the graph.
    ///
    /// Insert given edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    fn try_add_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier does not exists in the graph.
    ///
    fn try_del_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError>;
}

/// Vertex iterator.
/// 
/// Return the vertices iterator representing $V(G)$.
/// 
#[macro_export]
macro_rules! V {
    ($x:expr) => {
        $x.vertices_iter()
    };
}

/// Edge iterator.
/// 
/// Return the edges iterator representing $E(G)$.
/// 
#[macro_export]
macro_rules! E {
    ($x:expr) => {
        $x.edges_iter()
    };
}
