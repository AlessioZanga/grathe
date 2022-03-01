use super::{Capacity, Operators};
use crate::types::Error;
use crate::types::{EdgeIterator, VertexIterator, Vertex};
use std::fmt::Debug;

/// The graph storage trait.
pub trait Storage: Eq + PartialOrd + Default + Debug + Capacity + Operators {
    /// Vertex identifier type.
    type Vertex: Vertex;

    // TODO: Uncomment once associated type defaults are stable.
    // Edge identifier type.
    // type Edge = (&Self::Vertex, &Self::Vertex);

    /// Underlying storage type.
    type Storage;

    /// Return immutable reference to underlying raw storage.
    fn storage(&self) -> &Self::Storage;

    /// New constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The new constructor of $\mathcal{G}$
    /// returns a graph $G$ based on $V$ and $E$.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a new graph.
    /// let g = Graph::new((0..3), [(0, 1), (1, 2)]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 2);
    /// ```
    ///
    fn new<I, J, V>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        V: Into<Self::Vertex>;

    /// Null constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The null constructor of $\mathcal{G}$
    /// returns a null graph $G$ (i.e. both $V$ and $E$ are empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let g = Graph::null();
    ///
    /// // The vertex set is empty.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn null() -> Self;

    /// Empty constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The empty constructor of $\mathcal{G}$
    /// returns an empty graph $G$ (i.e. $E$ is empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build an empty graph.
    /// let g = Graph::empty(0..3);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn empty<I, V>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;

    /// Complete constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The complete constructor of $\mathcal{G}$
    /// returns an complete graph $G$ (i.e. $E$ is $V \times V$).
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a complete graph.
    /// let g = DiGraph::complete(0..3);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), g.order() * g.order());
    /// ```
    ///
    fn complete<I, V>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;

    /// Clears the graph.
    ///
    /// Clears the graph, removing both vertex and edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a new graph.
    /// let mut g = Graph::from_edges([(0, 1), (2, 3)]);
    ///
    /// // The graph *is not* null.
    /// assert_ne!(g.order(), 0);
    /// assert_ne!(g.size(), 0);
    ///
    /// // Clear the graph.
    /// g.clear();
    ///
    /// // The graph *is* null.
    /// assert_eq!(g.order(), 0);
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn clear(&mut self);

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $V$ ordered by identifier value.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_vertices(0..3);
    ///
    /// // Use the vertex set iterator.
    /// assert_true!(g.vertices_iter().eq(&[0, 1, 2]));
    ///
    /// // Use the associated macro 'V!'.
    /// assert_true!(g.vertices_iter().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert_true!(g.has_vertex(x));
    /// }
    /// ```
    ///
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $E$ order by identifier values.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a 3rd order graph.
    /// let g = Graph::from_edges([(0, 1), (1, 0)]);
    ///
    /// // Use the vertex set iterator.
    /// assert_true!(g.edges_iter().eq([(&0, &1), (&1, &0)]));
    ///
    /// // Use the associated macro 'E!'.
    /// assert_true!(g.edges_iter().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for (x, y) in E!(g) {
    ///     assert_true!(g.has_edge(&x, &y)?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a>;

    /// Adjacent iterator.
    ///
    /// Iterates over the vertex set $Adj(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error>
    /// # {
    /// // Build a graph from edges.
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// assert_true!(g.adjacents_iter(&0).eq(&[0, 1, 2]));
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert_true!(g.adjacents_iter(&0).eq(Adj!(g, &0)));
    ///
    /// // Iterate over the adjacent set.
    /// for &x in Adj!(g, &0) {
    ///     assert_true!(g.has_edge(&0, &x)?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
    ///
    /// # Complexity
    ///
    /// $O(1)$ - Constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a 5th order graph.
    /// let g = Graph::from_vertices(0..5);
    /// assert_eq!(g.order(), 5);
    /// ```
    ///
    fn order(&self) -> usize;

    /// Size of the graph.
    ///
    /// Return the graph size (aka. $|E|$).
    ///
    /// # Complexity
    ///
    /// $O(|E|)$ - Liner in the size of graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a 5th size graph.
    /// let g = Graph::from_edges([
    ///     (0, 1), (2, 0), (3, 2), (1, 2), (1, 1)
    /// ]);
    /// assert_eq!(g.size(), 5);
    /// ```
    ///
    fn size(&self) -> usize;

    /// Checks vertex in the graph.
    ///
    /// Checks whether the graph has a given vertex or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a 2nd order graph.
    /// let g = Graph::from_vertices(0..2);
    ///
    /// // Check vertex.
    /// assert_true!(g.has_vertex(&0));
    /// assert_false!(g.has_vertex(&2));
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex("0")?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_vertex(&i));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds vertex to the graph.
    ///
    /// Insert a new vertex identifier into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier already exists in the graph.
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
    /// // Add a new vertex.
    /// let i = g.add_vertex(0)?;
    /// assert_true!(g.has_vertex(&i));
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex("0")?;
    ///
    /// // Adding an existing vertex label yields an error.
    /// assert_true!(g.add_vertex("0").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_vertex<V>(&mut self, x: V) -> Result<Self::Vertex, Error<Self::Vertex>>
    where
        V: Into<Self::Vertex>;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
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
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// let i = g.add_vertex(0)?;
    /// assert_true!(g.has_vertex(&i));
    ///
    /// // Delete the newly added vertex.
    /// g.del_vertex(&i)?;
    /// assert_false!(g.has_vertex(&i));
    ///
    /// // Deleting a non-existing vertex yields an error.
    /// assert_true!(g.del_vertex(&i).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Check edge.
    /// assert_true!(g.has_edge(&0, &1)?);
    ///
    /// // Checking an edge with non-existing vertex yields an error.
    /// assert_true!(g.has_edge(&0, &42).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Add a edge given its label.
    /// let x = g.add_vertex("0")?;
    /// let y = g.add_vertex("1")?;
    /// g.add_edge(&x, &y)?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_edge(&x, &y)?);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, Error<Self::Vertex>>;

    /// Adds edge to the graph.
    ///
    /// Add new edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a 2nd order graph.
    /// let mut g = Graph::from_vertices(0..2);
    ///
    /// // Set vertices identifiers.
    /// let (x, y) = (0, 1);
    ///
    /// // Add a new edge from vertex.
    /// g.add_edge(&x, &y)?;
    /// assert_true!(g.has_edge(&x, &y)?);
    ///
    /// // Adding an existing edge yields an error.
    /// assert_true!(g.add_edge(&x, &y).is_err());
    ///
    /// // Build a 3rd order graph.
    /// let mut g = Graphl::from_vertices(
    ///     (0..3).into_iter().map(|x| x.to_string())
    /// );
    ///
    /// // Set vertices identifiers.
    /// let x: String = "0".into();
    /// let y: String = "1".into();
    ///
    /// // Add a edge given its label.
    /// g.add_edge(&x, &y)?;
    ///
    /// // Adding an existing edge label yields an error.
    /// assert_true!(g.add_edge(&x, &y).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the edge identifier does not exist in the graph.
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
    /// // Add a new edge.
    /// let x = g.add_vertex(0)?;
    /// let y = g.add_vertex(1)?;
    /// g.add_edge(&x, &y)?;
    /// assert_true!(g.has_edge(&x, &y)?);
    ///
    /// // Delete the newly added edge.
    /// g.del_edge(&x, &y)?;
    /// assert_false!(g.has_edge(&x, &y)?);
    ///
    /// // Deleting a non-existing edge yields an error.
    /// assert_true!(g.del_edge(&x, &y).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;

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
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
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
        self.adjacents_iter(x).count()
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
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
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
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
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

/// Vertex iterator.
///
/// Return the vertex iterator representing $V(G)$.
///
#[macro_export]
macro_rules! V {
    ($g:expr) => {
        $g.vertices_iter()
    };
}

/// Edge iterator.
///
/// Return the edges iterator representing $E(G)$.
///
#[macro_export]
macro_rules! E {
    ($g:expr) => {
        $g.edges_iter()
    };
}

/// Adjacency iterator.
///
/// Return the vertex iterator representing $Adj(G, X)$.
///
#[macro_export]
macro_rules! Adj {
    ($g:expr, $x:expr) => {
        $g.adjacents_iter($x)
    };
}
