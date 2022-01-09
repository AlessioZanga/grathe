use crate::traits::{Capacity, Operators, Storage, WithAttributes};
use crate::Adj;

/// The base graph trait.
pub trait Base: Capacity + Operators + Storage + WithAttributes {
    /// Builds subgraph from given vertices.
    ///
    /// Builds a subgraph, preserving edges between given vertices.
    /// Ignores additional attributes (for now).
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifiers do not exist in the graph.
    ///
    fn subgraph<'a, I>(&self, iter: I) -> Self
    where
        Self: 'a,
        I: IntoIterator<Item = &'a Self::Vertex>,
    {
        // Build a subgraph from the given vertices.
        let mut subgraph = Self::from_vertices(iter);
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
    /// let g = Graph::new();
    /// let h = Graph::from_order(2);
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
    /// let g = Graph::new();
    /// let h = Graph::from_order(2);
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

    /// Degree of vertex.
    ///
    /// Degree of given vertex identifier $E$ as $|Adj(G, X)|$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Get the degree of `1`.
    /// assert_true!(
    ///     g.degree_of(&1) == 3 &&
    ///     g.degree_of(&1) == Adj!(g, &1).count()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn degree_of(&self, x: &Self::Vertex) -> usize {
        Adj!(self, x).count()
    }

    /// Is isolated vertex.
    ///
    /// Checks whether the vertex is not adjacent to any other vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is isolated (a.k.a not connected).
    /// assert_false!(g.is_isolated_vertex(&0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_isolated_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree_of(x) == 0
    }

    /// Is pendant vertex.
    ///
    /// Checks whether the vertex is adjacent to only one vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is pendant (a.k.a is connected to just one vertex).
    /// assert_true!(g.is_pendant_vertex(&0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree_of(x) == 1
    }
}

impl<T> Base for T where T: Capacity + Operators + Storage + WithAttributes {}
