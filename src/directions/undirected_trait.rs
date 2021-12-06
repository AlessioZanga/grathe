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
    ) -> Result<Box<dyn VertexIterator<Self::Vertex> + 'a>, Error<Self::Vertex>>;

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
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;

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
    fn reserve_undirected_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.reserve_vertex(&e.0)?;
        self.reserve_vertex(&e.1)?;
        self.add_undirected_edge(e)
    }

    /// Adds undirected edge label to the graph.
    ///
    /// Insert given undirected edge label into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge label already exists in the graph.
    ///
    #[inline(always)]
    fn add_undirected_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.add_undirected_edge(x)?;
        self.set_edge_label(x, y)
    }

    /// Adds undirected edge label to the graph.
    ///
    /// Insert given vertex identifiers, undirected edge identifier and
    /// undirected edge label into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers already exists in the graph,
    /// or the edge identifier or label already exists in the graph.
    ///
    #[inline(always)]
    fn reserve_undirected_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.reserve_undirected_edge(x)?;
        self.set_edge_label(x, y)
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
