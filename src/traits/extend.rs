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
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Extend graph with vertices.
    /// g.extend_vertices([0, 3, 1, 2])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Extending with existing vertices yields an error.
    /// assert_true!(g.extend_vertices([0]).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Extend graph with vertices.
    /// g.extend_vertices(["0", "3", "1", "2"])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Extending with existing vertices yields an error.
    /// assert_true!(g.extend_vertices(["0"]).is_err());
    /// # Ok(())
    /// # }
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
        // Add vertex to the graph.
        iter.map(|x| self.add_vertex(x)).fold(false, |acc, x| acc || x)
    }

    /// Extends graph with given edges.
    ///
    /// Extends graph with given sequence of edges identifiers.
    /// Non-existing vertices will be added as well.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Extend graph with edges.
    /// g.extend_edges([(0, 3), (1, 2)])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edges([(0, 3)]).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Extend graph with edges.
    /// g.extend_edges([("0", "3"), ("1", "2")])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edges([("0", "3")]).is_err());
    /// # Ok(())
    /// # }
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
        // Add edge to the graph.
        iter.map(|(x, y)| self.add_edge(&x, &y)).fold(false, |acc, x| acc || x)
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
