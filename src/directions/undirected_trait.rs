use crate::directions::DirectionalTrait;
use crate::errors::Error;
use crate::storages::StorageTrait;
use crate::types::*;

/// Undirected graph trait.
pub trait UndirectedTrait: DirectionalTrait + StorageTrait {
    /// Neighbor iterator.
    ///
    /// Iterates over the vertex set $Ne(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn neighbors_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
}

/// Neighborhood iterator.
///
/// Return the vertex iterator representing $Ne(G, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.neighbors_iter($x)
    };
}
