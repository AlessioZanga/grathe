use super::Storage;

/// The graph extend trait.
pub trait Extend: Storage {
    /// Extends graph with given vertices.
    ///
    /// Extends graph with given sequence of vertex identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Extend graph with vertices.
    /// assert!(g.extend_vertices([0, 3, 1, 2]));
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Extending with existing vertices return false.
    /// assert!(!g.extend_vertices([0]));
    /// ```
    ///
    fn extend_vertices<I>(&mut self, iter: I) -> bool
    where
        I: IntoIterator<Item = Self::Vertex>,
    {
        // Get vertex iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add vertex to the graph. NOTE: Do not replace with `any`, since it is a short circuit implementation.
        iter.fold(false, |acc, x| acc | self.add_vertex(x))
    }

    /// Extends graph with given edges.
    ///
    /// Extends graph with given sequence of edges identifiers.
    /// 
    /// # Panics
    /// 
    /// Panics if at least one vertex is not in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build an empty graph.
    /// let mut g = Graph::empty([0, 1, 2, 3]);
    ///
    /// // Extend graph with edges.
    /// assert!(g.extend_edges([(0, 3), (1, 2)]));
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges return false.
    /// assert!(!g.extend_edges([(0, 3)]));
    /// ```
    ///
    fn extend_edges<I>(&mut self, iter: I) -> bool
    where
        I: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Get edge iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add edge to the graph. NOTE: Do not replace with `any`, since it is a short circuit implementation.
        iter.fold(false, |acc, (x, y)| acc | self.add_edge(&x, &y))
    }
}

macro_rules! impl_extend {
    ($graph: ident) => {
        impl<V, A> $crate::traits::Extend for $graph<V, A>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
        }
    };
}

pub(crate) use impl_extend;
