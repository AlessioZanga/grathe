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
    fn neighbors_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    fn add_undirected_edge(
        &mut self,
        x: &Self::Vertex,
        y: &Self::Vertex,
    ) -> Result<(), Error<Self::Vertex>>;

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
    fn reserve_undirected_edge<T>(
        &mut self,
        x: &T,
        y: &T,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>
    where
        T: Eq + Clone + Into<Self::Vertex>,
    {
        let x = self.add_vertex(x)?;
        let y = self.add_vertex(y)?;
        self.add_undirected_edge(&x, &y)?;
        Ok((x, y))
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
