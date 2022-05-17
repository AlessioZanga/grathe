use super::{Capacity, Storage};

/// The graph from trait.
pub trait From: Capacity + Storage {
    /// From vertex constructor.
    ///
    /// Construct a graph from a given sequence of vertex, ignoring repeated ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique vertex.
    /// let vertices = [0, 3, 1, 2];
    ///
    /// // Build a graph given a vector of vertex.
    /// let g = Graph::from_vertices(vertices);
    ///
    /// // Build a graph given any `IntoIterator`.
    /// let h = Graph::from_vertices(0..4);
    ///
    /// assert_eq!(g, h);
    /// ```
    ///
    fn from_vertices<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        // Get vertex iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Build graph with initial capacity.
        let mut g = Self::with_capacity(lower);
        // Add vertex to the graph.
        iter.for_each(|x| {
            g.add_vertex(x);
        });

        g
    }

    /// From edges constructor.
    ///
    /// Construct a graph from a given sequence of edges, ignoring repeated ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique edges.
    /// let edges = [(0, 1), (2, 3), (1, 2)];
    ///
    /// // Build a graph given a vector of edges.
    /// let g = Graph::from_edges(edges);
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 3);
    /// ```
    ///
    fn from_edges<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Get edges iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Build graph with initial capacity.
        let mut g = Self::with_capacity(lower);
        // Add edges to the graph.
        iter.for_each(|(x, y)| {
            g.add_vertex(x);
            g.add_vertex(y);
            g.add_edge(&x, &y);
        });

        g
    }
}

macro_rules! impl_from {
    ($graph: ident) => {
        impl<V, A> $crate::traits::From for $graph<V, A>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
        }
    };
}

pub(crate) use impl_from;
