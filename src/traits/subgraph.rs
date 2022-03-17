use crate::traits::Storage;

pub trait Subgraph: PartialOrd + Storage {
    /// Builds subgraph from given vertices.
    ///
    /// Builds a subgraph, preserving edges between given vertices.
    /// Ignores additional attributes (for now).
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifiers do not exist in the graph.
    ///
    fn subgraph<I, V>(&self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>,
    {
        // Build a subgraph from the given vertices.
        let mut subgraph: Self = Default::default();
        for x in iter {
            subgraph.add_vertex(x).ok();
        }
        // Check if is it a proper subgraph of self,
        // i.e. given vertices are contained in self.
        assert!(subgraph.is_subgraph(self));
        // Copy edges into subgraph.
        for (x, y) in self.edges_iter() {
            subgraph.add_edge(x, y).ok();
        }

        subgraph
    }

    /// Is subgraph of another graph.
    ///
    /// Checks if this graph is subgraph of given graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::null();
    /// let h = Graph::from_vertices(0..2);
    ///
    /// // The null graph is always subgraph of an other graph.
    /// assert_true!(g.is_subgraph(&h));
    ///
    /// // Use the associated `<=` operator.
    /// assert_true!(g <= h);
    /// ```
    ///
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Is supergraph of another graph.
    ///
    /// Checks if this graph is supergraph of given graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::null();
    /// let h = Graph::from_vertices(0..2);
    ///
    /// // Any graph is supergraph of the null graph.
    /// assert_true!(h.is_supergraph(&g));
    ///
    /// // Use the associated `>=` operator.
    /// assert_true!(h >= g);
    /// ```
    ///
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }
}

macro_rules! impl_subgraph {
    ($graph:ident) => {
        impl<V, A> $crate::traits::Subgraph for $graph<V, A>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
        }
    };
}

pub(crate) use impl_subgraph;
