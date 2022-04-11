use super::{Capacity, Operators};
use crate::types::{EdgeIterator, Vertex, VertexIterator};
use std::fmt::Debug;

/// The graph storage trait.
pub trait Storage: Capacity + Debug + Default + Eq + Operators + PartialOrd {
    /// Vertex identifier type.
    type Vertex: Vertex;

    // TODO: Uncomment once associated type defaults are stable.
    // Edge identifier type.
    // type Edge = (&Self::Vertex, &Self::Vertex);

    /// Direction identifier type.
    type Direction;

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
    fn new<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex)>;

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
    fn empty<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>;

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
    fn complete<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>;

    /// Clears the graph.
    ///
    /// Clears the graph, removing both vertex and edges.
    ///
    /// # Examples
    ///
    /// ```
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
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_vertices(0..3);
    ///
    /// // Use the vertex set iterator.
    /// assert!(g.vertices_iter().eq(&[0, 1, 2]));
    ///
    /// // Use the associated macro 'V!'.
    /// assert!(g.vertices_iter().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert!(g.has_vertex(x));
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
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Use the vertex set iterator.
    /// assert!(g.edges_iter().eq([(&0, &1), (&2, &3)]));
    ///
    /// // Use the associated macro 'E!'.
    /// assert!(g.edges_iter().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for (x, y) in E!(g) {
    ///     assert!(g.has_edge(&x, &y));
    /// }
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
    /// use grathe::prelude::*;
    ///
    /// // Build a graph from edges.
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// assert!(g.adjacents_iter(&0).eq(&[0, 1, 2]));
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert!(g.adjacents_iter(&0).eq(Adj!(g, &0)));
    ///
    /// // Iterate over the adjacent set.
    /// for &x in Adj!(g, &0) {
    ///     assert!(g.has_edge(&0, &x));
    /// }
    /// ```
    ///
    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
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
    /// use grathe::prelude::*;
    ///
    /// // Build a 2nd order graph.
    /// let g = Graph::from_vertices(0..2);
    ///
    /// // Check vertices.
    /// assert!(g.has_vertex(&0));
    /// assert!(g.has_vertex(&1));
    /// assert!(!g.has_vertex(&2));
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
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// assert!(g.add_vertex(0));
    /// assert!(g.has_vertex(&0));
    /// ```
    ///
    fn add_vertex(&mut self, x: Self::Vertex) -> bool;

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
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Add a new vertex.
    /// assert!(g.add_vertex(0));
    /// assert!(g.has_vertex(&0));
    ///
    /// // Delete the newly added vertex.
    /// assert!(g.del_vertex(&0));
    /// assert!(!g.has_vertex(&0));
    ///
    /// // Deleting a non-existing vertex return false.
    /// assert!(!g.del_vertex(&0));
    /// ```
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> bool;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Panics
    ///
    /// Panics if at least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Check edge.
    /// assert!(g.has_edge(&0, &1));
    /// ```
    ///
    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Adds edge to the graph.
    ///
    /// Add new edge identifier into the graph.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a 2nd order graph.
    /// let mut g = Graph::from_vertices(0..2);
    ///
    /// // Add a new edge from vertex.
    /// assert!(g.add_edge(&0, &1));
    /// assert!(g.has_edge(&0, &1));
    ///
    /// // Adding an existing edge return false.
    /// assert!(!g.add_edge(&0, &1));
    /// ```
    ///
    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Panics
    ///
    /// Panics if at least one of the vertex identifiers does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Delete an edge.
    /// assert!(g.del_edge(&0, &1));
    /// assert!(!g.has_edge(&0, &1));
    ///
    /// // Deleting a non-existing edge return false.
    /// assert!(!g.del_edge(&0, &1));
    /// ```
    ///
    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

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
    /// use grathe::prelude::*;
    ///
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Get the degree of `1`.
    /// assert!(
    ///     g.degree(&1) == 3 &&
    ///     g.degree(&1) == Adj!(g, &1).count()
    /// );
    /// ```
    ///
    fn degree(&self, x: &Self::Vertex) -> usize {
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
    /// use grathe::prelude::*;
    ///
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is isolated (a.k.a not connected).
    /// assert!(!g.is_isolated_vertex(&0));
    /// ```
    ///
    fn is_isolated_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree(x) == 0
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
    /// use grathe::prelude::*;
    ///
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is pendant (a.k.a is connected to just one vertex).
    /// assert!(g.is_pendant_vertex(&0));
    /// ```
    ///
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree(x) == 1
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
