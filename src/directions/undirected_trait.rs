use crate::errors::Error;
use crate::storages::StorageTrait;
use crate::types::*;

/// Undirected graph trait.
pub trait UndirectedTrait: StorageTrait {
    /// Neighbor iterator.
    ///
    /// Iterates over the neighbors vertex set $Ne(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exists in the graph.
    ///
    fn neighbors_iter<'a, T>(
        &'a self,
        x: &'a T,
    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>
    where
        T: Eq + Clone + Into<Self::Vertex>;

    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    fn add_undirected_edge<'a, T>(
        &mut self,
        x: (&'a T, &'a T),
    ) -> Result<(&'a T, &'a T), Error<Self::Vertex>>
    where
        T: Eq + Clone + Into<Self::Vertex>;

    /// Adds undirected edge to the graph.
    ///
    /// Insert given vertex identifiers and undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers already exists in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    #[inline(always)]
    fn reserve_undirected_edge<'a, T>(
        &mut self,
        (x, y): (&'a T, &'a T),
    ) -> Result<(&'a T, &'a T), Error<Self::Vertex>>
    where
        T: Eq + Clone + Into<Self::Vertex>,
    {
        self.add_vertex(x)?;
        self.add_vertex(y)?;
        self.add_undirected_edge((x, y))
    }
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
