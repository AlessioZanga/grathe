use crate::{
    traits::{Directed, PartiallyDirected, Storage},
    types::{directions, VertexIterator},
    E, V,
};

/// Undirected graph trait.
pub trait Undirected: Storage {
    /// Constructs from another undirected graph.
    fn from_undirected<G>(other: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex, Direction = directions::Undirected>,
    {
        Self::new(V!(other).cloned(), E!(other).map(|(x, y)| (x.clone(), y.clone())))
    }

    /// Constructs from a directed graph by making any edge undirected.
    fn from_directed<G>(other: G) -> Self
    where
        G: Directed<Vertex = Self::Vertex, Direction = directions::Directed>,
    {
        Self::new(V!(other).cloned(), E!(other).map(|(x, y)| (x.clone(), y.clone())))
    }

    /// Constructs from a partially-directed graph by making any edge undirected.
    fn from_partially_directed<G>(other: G) -> Self
    where
        G: PartiallyDirected<Vertex = Self::Vertex, Direction = directions::PartiallyDirected>,
    {
        Self::new(V!(other).cloned(), E!(other).map(|(x, y)| (x.clone(), y.clone())))
    }

    /// Neighbor iterator.
    ///
    /// Iterates over the vertex set $Ne(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn neighbors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
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
