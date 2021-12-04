use crate::errors::Error;
use crate::types::*;
use crate::{Adj, E, V};
use nasparse::CooMatrix;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fmt::Debug;

/// The base graph storage trait.
pub trait StorageTrait: Eq + PartialOrd + Default + Debug {
    /// Vertex identifier type.
    // TODO: Change FromPrimitive to Step once stable, use combination of x = T::default()
    // and x = Step::forward(x, 1) to increase Vertex in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type Vertex: VertexTrait;

    // TODO: Uncomment once associated type defaults are stable.
    // Edge identifier type.
    // type Edge = (Self::Vertex, Self::Vertex);

    /// Underlying storage type.
    type Storage;

    /// Base constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The base constructor of $\mathcal{G}$
    /// returns a null graph $G$ (i.e. both $V$ and $E$ are empty).
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let g = Graph::new();
    ///
    /// // The vertex set is empty.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn new() -> Self;

    /// Clears the graph.
    ///
    /// Clears the graph, removing both vertex and edges.
    ///
    /// # Examples
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

    /// Returns the capacity.
    ///
    /// Returns the number of vertex the graph can hold.
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a graph with a specific capacity.
    /// let g = Graph::with_capacity(3);
    /// assert_eq!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn capacity(&self) -> usize;

    /// With capacity constructor.
    ///
    /// Construct a graph of a given capacity (a.k.a. order).
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a graph with a specific capacity.
    /// let g = Graph::with_capacity(3);
    /// assert_eq!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn with_capacity(capacity: usize) -> Self;

    /// Reserves additional capacity.
    ///
    /// Reserves capacity for at least `additional` vertex to be inserted in the graph.
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the additional capacity overflows `usize`.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Reserve additional capacity.
    /// g.reserve(3);
    /// // FIXME: assert_eq!(g.capacity(), 3);
    ///
    /// // The order is still zero.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The size is still zero.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn reserve(&mut self, additional: usize);

    /// Shrinks the capacity with a lower limit.
    ///
    /// Shrinks the capacity of the graph with a lower limit.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    /// // FIXME: assert_eq!(g.capacity(), 100);
    ///
    /// // Shrink capacity to given minimum.
    /// g.shrink_to(50);
    /// // FIXME: assert_eq!(g.capacity(), 50);
    /// ```
    ///
    fn shrink_to(&mut self, min_capacity: usize);

    /// Shrinks the capacity.
    ///
    /// Shrinks the capacity of the graph as much as possible.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    /// // FIXME: assert_eq!(g.capacity(), 100);
    ///
    /// // Shrink capacity as much as possible.
    /// g.shrink_to_fit();
    /// // FIXME: assert_eq!(g.capacity(), 0);
    /// ```
    ///
    fn shrink_to_fit(&mut self);

    /// From order constructor.
    ///
    /// Construct a graph of a given order.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_order(3);
    ///
    /// // There are three vertex...
    /// assert_eq!(g.order(), 3);
    ///
    /// // ... but no edges.
    /// assert_eq!(g.size(), 0);
    ///
    /// // Vertex identifiers are added incrementally starting from zero.
    /// assert_true!(g.has_vertex(&2));
    /// ```
    ///
    fn from_order(order: usize) -> Self {
        Self::from_vertex((0..order).map(|x| Self::Vertex::from_usize(x).unwrap()))
    }

    /// From vertex constructor.
    ///
    /// Construct a graph from a given sequence of vertex, ignoring repeated ones.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique vertex.
    /// let sequence = vec![0, 3, 1, 2];
    ///
    /// // Build a graph by consuming a vector of vertex.
    /// let g = Graph::from_vertex(sequence);
    ///
    /// // Build a graph by consuming any `IntoIterator`.
    /// let h = Graph::from_vertex((0..4));
    ///
    /// assert_eq!(g, h);
    ///
    /// ```
    ///
    fn from_vertex<Iter>(vertex: Iter) -> Self
    where
        Iter: IntoIterator<Item = Self::Vertex>,
    {
        // Get vertex iterator.
        let vertex = vertex.into_iter();
        // Get lower bound size hint.
        let (lower, _) = vertex.size_hint();
        // Build graph with initial capacity.
        let mut g = Self::with_capacity(lower);
        // Add vertex to the graph.
        for x in vertex {
            g.reserve_vertex(&x).ok();
        }

        g
    }

    /// From edges constructor.
    ///
    /// Construct a graph from a given sequence of edges, ignoring repeated ones.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique edges.
    /// let sequence = vec![(0, 1), (2, 3), (1, 2)];
    ///
    /// // Build a graph by consuming a vector of edges.
    /// let g = Graph::from_edges(sequence);
    ///
    /// // Build a graph by consuming any `IntoIterator`.
    /// let h = Graph::from_edges((0..4).zip((1..4)));
    ///
    /// assert_eq!(g, h);
    ///
    /// ```
    ///
    fn from_edges<Iter>(edges: Iter) -> Self
    where
        Iter: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        // Get edges iterator.
        let edges = edges.into_iter();
        // Get lower bound size hint.
        let (lower, _) = edges.size_hint();
        // Build graph with initial capacity,
        // assuming average frequency of new vertex.
        let mut g = Self::with_capacity(lower);
        // Add edges to the graph.
        for (x, y) in edges {
            g.reserve_vertex(&x).ok();
            g.reserve_vertex(&y).ok();
            g.add_edge(&(x, y)).ok();
        }

        g
    }

    /// Edge list adapter.
    ///
    /// Return the edge list representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|E|)$ - Linear in the size of the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of uniques edges.
    /// let E = EdgeList::from([(0, 1), (1, 0)]);
    ///
    /// // Build a graph from a sequence of edges.
    /// let g = Graph::from_edges(E.clone());
    ///
    /// // Return an edge list (a.k.a. a *set* of edges) from the graph.
    /// assert_eq!(g.to_edge_list(), E);
    /// ```
    ///
    fn to_edge_list(&self) -> EdgeList<Self::Vertex> {
        let mut out = EdgeList::new();
        out.extend(E!(self));
        out
    }

    /// Adjacency list adapter.
    ///
    /// Return the adjacency list representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    fn to_adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
        let mut out = AdjacencyList::new();
        for (x, y) in E!(self) {
            out.entry(x).or_default().insert(y);
        }
        out
    }

    /// Dense adjacency matrix adapter.
    ///
    /// Return the (dense) adjacency matrix representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V|^2)$ - Quadratic in the order of the graph.
    ///
    fn to_dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::new();
        let mut out = DenseAdjacencyMatrix::zeros(n, n);
        // Build vid-to-index mapping.
        idx.extend(
            V!(self)
                .into_iter()
                .enumerate()
                .map(|(index, vid)| (vid, index)),
        );
        // Populate the output value.
        for (x, y) in E!(self) {
            out[(idx[&x], idx[&y])] = 1;
        }
        out
    }

    /// Sparse adjacency matrix adapter.
    ///
    /// Return the (sparse) adjacency matrix representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    fn to_sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::new();
        let mut out = CooMatrix::<i8>::zeros(n, n);
        // Build vid-to-index mapping.
        idx.extend(
            V!(self)
                .into_iter()
                .enumerate()
                .map(|(index, vid)| (vid, index)),
        );
        // Populate the output value.
        out.reserve(self.size());
        for (x, y) in E!(self) {
            out.push(idx[&x], idx[&y], 1);
        }
        SparseAdjacencyMatrix::from(&out)
    }

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $V$ ordered by identifier value.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_order(3);
    ///
    /// // Use the vertex set iterator.
    /// let V: Vec<_> = g.vertex_iter().collect();
    /// assert_eq!(V, [0, 1, 2]);
    ///
    /// // Use the associated macro 'V!'.
    /// assert_true!(g.vertex_iter().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert_true!(g.has_vertex(&x));
    /// }
    /// ```
    ///
    fn vertex_iter<'a>(&'a self) -> Box<dyn VertexIterator<Self::Vertex> + 'a>;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $E$ order by identifier values.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a 3rd order graph.
    /// let g = Graph::from_edges([(0, 1), (1, 0)]);
    ///
    /// // Use the vertex set iterator.
    /// let E: Vec<_> = g.edges_iter().collect();
    /// assert_eq!(E, [(0, 1), (1, 0)]);
    ///
    /// // Use the associated macro 'E!'.
    /// assert_true!(g.edges_iter().eq(E!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in E!(g) {
    ///     assert_true!(g.has_edge(&x)?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<Self::Vertex> + 'a>;

    /// Adjacent iterator.
    ///
    /// Iterates over the adjacent vertex set $Adj(G, X)$ of a given vertex $X$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>>
    /// # {
    /// // Build a graph from edges.
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// let A: Vec<_> = g.adjacent_iter(&0)?.collect();
    /// assert_eq!(A, [0, 1, 2]);
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert_true!(g.adjacent_iter(&0)?.eq(Adj!(g, &0)?));
    ///
    /// // Iterate over the adjacent set.
    /// for x in Adj!(g, &0)? {
    ///     assert_true!(g.has_edge(&(0, x))?);
    /// }
    ///
    /// // Iterating over non-existing vertex yields an error.
    /// assert_true!(Adj!(g, &3).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn adjacent_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// vertex labels.
    ///
    /// Return immutable reference to internal vertex labels.
    ///
    fn as_vertex_labels(&self) -> &LabelMap<Self::Vertex>;

    /// Mutable vertex labels.
    ///
    /// Return mutable reference to internal vertex labels.
    ///
    fn as_mut_vertex_labels(&mut self) -> &mut LabelMap<Self::Vertex>;

    /// Edges labels.
    ///
    /// Return immutable reference to internal edges labels.
    ///
    fn as_edges_labels(&self) -> &LabelMap<(Self::Vertex, Self::Vertex)>;

    /// Mutable edges labels.
    ///
    /// Return mutable reference to internal edges labels.
    ///
    fn as_mut_edges_labels(&mut self) -> &mut LabelMap<(Self::Vertex, Self::Vertex)>;

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
    ///
    /// # Complexity
    ///
    /// $O(1)$ - Constant.
    ///
    /// # Examples
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a 5th order graph.
    /// let g = Graph::from_order(5);
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
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a 2nd order graph.
    /// let g = Graph::from_order(2);
    ///
    /// // Check vertex.
    /// assert_true!(g.has_vertex(&0));
    /// assert_false!(g.has_vertex(&2));
    /// # Ok(())
    /// # }
    /// ```
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
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex.
    /// let i = g.add_vertex()?;
    /// assert_true!(g.has_vertex(&i));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_vertex(&mut self) -> Result<Self::Vertex, Error<Self::Vertex>>;

    /// Adds vertex to the graph.
    ///
    /// Insert given vertex identifier into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex with a specific identifier.
    /// let i = g.reserve_vertex(&42)?;
    /// assert_true!(g.has_vertex(&i) && i == 42);
    ///
    /// // Adding an existing vertex yields an error.
    /// assert_true!(g.reserve_vertex(&42).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn reserve_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>>;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex.
    /// let i = g.add_vertex()?;
    /// assert_true!(g.has_vertex(&i));
    ///
    /// // Delete the newly added vertex.
    /// let j = g.del_vertex(&i)?;
    /// assert_false!(g.has_vertex(&i));
    ///
    /// // Deleting a vertex returns its identifier.
    /// assert_eq!(i, j);
    ///
    /// // Deleting a non-existing vertex yields an error.
    /// assert_true!(g.del_vertex(&i).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Check edge.
    /// assert_true!(g.has_edge(&(0, 1))?);
    ///
    /// // Checking an edge with non-existing vertex yields an error.
    /// assert_true!(g.has_edge(&(0, 42)).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, Error<Self::Vertex>>;

    /// Adds edge to the graph.
    ///
    /// Add new edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a 2nd order graph.
    /// let mut g = Graph::from_order(2);
    ///
    /// // Add a new edge from vertex.
    /// let e = g.add_edge(&(0, 1))?;
    /// assert_true!(g.has_edge(&e)?);
    ///
    /// // Adding an existing edge yields an error.
    /// assert_true!(g.add_edge(&e).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;

    /// Adds edge to the graph.
    ///
    /// Insert given vertex identifiers and edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers already exists in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge with specific identifiers.
    /// let e = g.reserve_edge(&(0, 1))?;
    /// assert_true!(
    ///     g.has_vertex(&0) &&
    ///     g.has_vertex(&1) &&
    ///     g.has_edge(&e)?
    /// );
    ///
    /// // Reserving an existing edge yields an error.
    /// assert_true!(g.reserve_edge(&e).is_err());
    ///
    /// // Also, reserving a non-existing edge with.
    /// // at least one existing vertex yields an error...
    /// assert_true!(g.reserve_edge(&(0, 2)).is_err());
    ///
    /// // ..but adding them is fine.
    /// g.reserve_vertex(&2)?;
    /// assert_false!(g.add_edge(&(0, 2)).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn reserve_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.reserve_vertex(&e.0)?;
        self.reserve_vertex(&e.1)?;
        self.add_edge(e)
    }

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge.
    /// let e = g.reserve_edge(&(0, 1))?;
    /// assert_true!(g.has_edge(&e)?);
    ///
    /// // Delete the newly added edge.
    /// let f = g.del_edge(&e)?;
    /// assert_false!(g.has_edge(&e)?);
    ///
    /// // Deleting a edge returns its identifier.
    /// assert_eq!(e, f);
    ///
    /// // Deleting a non-existing edge yields an error.
    /// assert_true!(g.del_edge(&e).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;

    /// Checks vertex label.
    ///
    /// Checks whether the graph has a given vertex label or not.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex_label("0")?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_vertex_label("0"));
    ///
    /// // Check whether an identifier is associated to a specific label (stable).
    /// assert_true!(matches!(g.get_vertex_label(&i), Ok("0")));
    ///
    /// // TODO: Check whether an identifier is associated to a specific label (nightly).
    /// // assert_true!(g.get_vertex_label(&i).contains("0"));
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn has_vertex_label(&self, x: &str) -> bool {
        self.as_vertex_labels().contains_right(x)
    }

    /// Adds vertex label to the graph.
    ///
    /// Insert given vertex label into the graph.
    ///
    /// # Errors
    ///
    /// The vertex label already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex_label("0")?;
    ///
    /// // Adding an existing vertex label yields an error.
    /// assert_true!(g.add_vertex_label("0").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn add_vertex_label(&mut self, y: &str) -> Result<Self::Vertex, Error<Self::Vertex>> {
        let x = self.add_vertex()?;
        self.set_vertex_label(&x, y)
    }

    /// Adds vertex label to the graph.
    ///
    /// Insert given vertex identifier and label into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier or label already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Reserve a vertex given its identifier and label.
    /// let i = g.reserve_vertex_label(&0, "0")?;
    /// assert_true!(
    ///     g.has_vertex(&i) &&                         // 0 in G
    ///     g.has_vertex_label("0") &&                  // "0" in G
    ///     matches!(g.get_vertex_label(&i), Ok("0"))   // (0, "0") in G
    /// );
    ///
    /// // Reserving an existing vertex identifier or label yields an error.
    /// assert_true!(g.reserve_vertex_label(&i, "0").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn reserve_vertex_label(
        &mut self,
        x: &Self::Vertex,
        y: &str,
    ) -> Result<Self::Vertex, Error<Self::Vertex>> {
        self.reserve_vertex(x)?;
        self.set_vertex_label(x, y)
    }

    /// Vertex identifier from label.
    ///
    /// Return vertex identifier given its label.
    ///
    /// # Errors
    ///
    /// The vertex label does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex_label("0")?;
    ///
    /// // Get vertex by label.
    /// let j = g.get_vertex_id("0")?;
    /// assert_eq!(i, j);
    ///
    /// // Getting non-existing vertex label yields an error.
    /// assert_true!(g.get_vertex_id("1").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn get_vertex_id(&self, x: &str) -> Result<Self::Vertex, Error<Self::Vertex>> {
        self.as_vertex_labels()
            .get_by_right(x)
            .copied()
            .ok_or_else(|| Error::VertexLabelNotDefined(String::from(x)))
    }

    /// Vertex label from identifier.
    ///
    /// Return vertex label given its identifier.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex_label("0")?;
    ///
    /// // Get vertex by label.
    /// let j = g.get_vertex_label(&i)?;
    /// assert_eq!("0", j);
    ///
    /// // Getting non-existing vertex label yields an error.
    /// assert_true!(g.get_vertex_label(&1).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn get_vertex_label(&self, x: &Self::Vertex) -> Result<&str, Error<Self::Vertex>> {
        self.as_vertex_labels()
            .get_by_left(x)
            .map(|x| x.as_str())
            .ok_or(Error::VertexNotDefined(*x))
    }

    /// Set vertex label.
    ///
    /// Sets vertex label given identifier.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph,
    /// or the vertex label is already defined.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex and set label.
    /// let i = g.add_vertex()?;
    /// g.set_vertex_label(&i, "0")?;
    /// assert_eq!(g.get_vertex_label(&i)?, "0");
    ///
    /// // Setting non-existing vertex label yields an error.
    /// assert_true!(g.set_vertex_label(&1, "1").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn set_vertex_label(
        &mut self,
        x: &Self::Vertex,
        y: &str,
    ) -> Result<Self::Vertex, Error<Self::Vertex>> {
        match self.has_vertex(x) {
            false => Err(Error::VertexNotDefined(*x)),
            true => match self.as_vertex_labels().contains_right(y) {
                false => {
                    // Overwrite previous label if present
                    self.as_mut_vertex_labels().insert(*x, String::from(y));
                    Ok(*x)
                }
                true => Err(Error::VertexLabelAlreadyDefined(String::from(y))),
            },
        }
    }

    /// Unset vertex label.
    ///
    /// Un-sets vertex label given identifier.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph,
    /// or the vertex label is not defined.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a vertex and set label.
    /// let i = g.add_vertex_label("0")?;
    /// assert_eq!(g.get_vertex_label(&i)?, "0");
    ///
    /// // Un-setting vertex label returns identifier-label pair.
    /// let (j, k) = g.unset_vertex_label(&i)?;
    /// assert_true!(
    ///     i == j &&
    ///     k == "0" &&
    ///     !g.has_vertex_label("0")
    /// );
    ///
    /// // Un-setting non-existing vertex label yields an error.
    /// assert_true!(g.unset_vertex_label(&1).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn unset_vertex_label(
        &mut self,
        x: &Self::Vertex,
    ) -> Result<(Self::Vertex, String), Error<Self::Vertex>> {
        self.as_mut_vertex_labels()
            .remove_by_left(x)
            .ok_or(Error::VertexNotDefined(*x))
    }

    /// Checks edge label.
    ///
    /// Checks whether the graph has a given edge label or not.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a edge given its label.
    /// let e = g.reserve_edge_label(&(0, 1), "(0, 1)")?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_edge_label("(0, 1)"));
    ///
    /// // Check whether an identifier is associated to a specific label (stable).
    /// assert_true!(matches!(g.get_edge_label(&e), Ok("(0, 1)")));
    ///
    /// // TODO: Check whether an identifier is associated to a specific label (nightly).
    /// // assert_true!(g.get_edge_label(&e).contains("(0, 1)"));
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn has_edge_label(&self, x: &str) -> bool {
        self.as_edges_labels().contains_right(x)
    }

    /// Adds edge label to the graph.
    ///
    /// Insert given edge label into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge label already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a 3rd order graph.
    /// let mut g = Graph::from_order(3);
    ///
    /// // Add a edge given its label.
    /// let e = g.add_edge_label(&(0, 1), "(0, 1)")?;
    ///
    /// // Adding an existing edge label yields an error.
    /// assert_true!(g.add_edge_label(&(0, 2), "(0, 1)").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn add_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.add_edge(x)?;
        self.set_edge_label(x, y)
    }

    /// Adds edge label to the graph.
    ///
    /// Insert given vertex identifiers, edge identifier and edge label into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers already exists in the graph,
    /// or the edge identifier or label already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Reserve a edge given its identifier and label.
    /// let e = g.reserve_edge_label(&(0, 1), "(0, 1)")?;
    /// assert_true!(
    ///     g.has_edge(&e)? &&                              // (0, 1) in G
    ///     g.has_edge_label("(0, 1)") &&                   // "(0, 1)" in G
    ///     matches!(g.get_edge_label(&e), Ok("(0, 1)"))    // ((0, 1), "(0, 1)") in G
    /// );
    ///
    /// // Reserving an existing edge identifier or label yields an error.
    /// assert_true!(g.reserve_edge_label(&e, "(0, 1)").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn reserve_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.reserve_edge(x)?;
        self.set_edge_label(x, y)
    }

    /// Edge identifier from label.
    ///
    /// Return edge identifier given its label.
    ///
    /// # Errors
    ///
    /// The edge label does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a edge given its label.
    /// let e = g.reserve_edge_label(&(0, 1), "(0, 1)")?;
    ///
    /// // Get edge by label.
    /// let f = g.get_edge_id("(0, 1)")?;
    /// assert_eq!(e, f);
    ///
    /// // Getting non-existing edge label yields an error.
    /// assert_true!(g.get_edge_id("(1, 2)").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn get_edge_id(&self, x: &str) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.as_edges_labels()
            .get_by_right(x)
            .copied()
            .ok_or_else(|| Error::EdgeLabelNotDefined(String::from(x)))
    }

    /// Edge label from identifier.
    ///
    /// Return edge label given its identifier.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a edge given its label.
    /// let e = g.reserve_edge_label(&(0, 1), "(0, 1)")?;
    ///
    /// // Get edge by label.
    /// let f = g.get_edge_label(&e)?;
    /// assert_eq!("(0, 1)", f);
    ///
    /// // Getting non-existing edge label yields an error.
    /// assert_true!(g.get_edge_label(&(1, 2)).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn get_edge_label(
        &self,
        x: &(Self::Vertex, Self::Vertex),
    ) -> Result<&str, Error<Self::Vertex>> {
        self.as_edges_labels()
            .get_by_left(x)
            .map(|x| x.as_str())
            .ok_or(Error::EdgeNotDefined(*x))
    }

    /// Set edge label.
    ///
    /// Sets edge label given identifier.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exists in the graph,
    /// or the edge label is already defined.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a edge and set label.
    /// let e = g.reserve_edge(&(0, 1))?;
    /// g.set_edge_label(&e, "(0, 1)")?;
    /// assert_eq!(g.get_edge_label(&e)?, "(0, 1)");
    ///
    /// // Setting non-existing edge label yields an error.
    /// assert_true!(g.set_edge_label(&(1, 2), "(1, 2)").is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn set_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        match self.has_edge(x)? {
            false => Err(Error::EdgeNotDefined(*x)),
            true => match self.as_edges_labels().contains_right(y) {
                false => {
                    // Overwrite previous label if present
                    self.as_mut_edges_labels().insert(*x, String::from(y));
                    Ok(*x)
                }
                true => Err(Error::EdgeLabelAlreadyDefined(String::from(y))),
            },
        }
    }

    /// Unset edge label.
    ///
    /// Un-sets edge label given identifier.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exists in the graph,
    /// or the edge label is not defined.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a edge and set label.
    /// let e = g.reserve_edge_label(&(0, 1), "(0, 1)")?;
    /// assert_eq!(g.get_edge_label(&e)?, "(0, 1)");
    ///
    /// // Un-setting edge label returns identifier-label pair.
    /// let (f, d) = g.unset_edge_label(&e)?;
    /// assert_true!(
    ///     e == f &&
    ///     d == "(0, 1)" &&
    ///     !g.has_edge_label("(0, 1)")
    /// );
    ///
    /// // Un-setting non-existing edge label yields an error.
    /// assert_true!(g.unset_edge_label(&(1, 2)).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn unset_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
    ) -> Result<((Self::Vertex, Self::Vertex), String), Error<Self::Vertex>> {
        self.as_mut_edges_labels()
            .remove_by_left(x)
            .ok_or(Error::EdgeNotDefined(*x))
    }

    /// Is subgraph of another graph.
    ///
    /// Checks if this graph is subgraph of given graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::default();
    /// let h = Graph::from_order(2);
    ///
    /// // The null graph is always subgraph of an other graph.
    /// assert_true!(g.is_subgraph(&h));
    ///
    /// // Use the associated `<=` operator.
    /// assert_true!(g <= h);
    /// ```
    ///
    #[inline(always)]
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Is supergraph of another graph.
    ///
    /// Checks if this graph is supergraph of given graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::default();
    /// let h = Graph::from_order(2);
    ///
    /// // Any graph is supergraph of the null graph.
    /// assert_true!(h.is_supergraph(&g));
    ///
    /// // Use the associated `>=` operator.
    /// assert_true!(h >= g);
    /// ```
    ///
    #[inline(always)]
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }

    /// Degree of vertex.
    ///
    /// Degree of given vertex identifier $X$ as $|Adj(G, X)|$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Get the degree of `1`.
    /// assert_true!(
    ///     g.degree_of(&1)? == 3 &&
    ///     g.degree_of(&1)? == Adj!(g, &1)?.count()
    /// );
    ///
    /// // Getting the degree of a non-existing vertex yields an error.
    /// assert_true!(g.degree_of(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn degree_of(&self, x: &Self::Vertex) -> Result<usize, Error<Self::Vertex>> {
        Ok(Adj!(self, x)?.count())
    }

    /// Is isolated vertex.
    ///
    /// Checks whether the vertex is not adjacent to any other vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is isolated (a.k.a not connected).
    /// assert_false!(g.is_isolated_vertex(&0)?);
    ///
    /// // Checking a non-existing vertex yields an error.
    /// assert_true!(g.is_isolated_vertex(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn is_isolated_vertex(&self, x: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        Ok(self.degree_of(x)? == 0)
    }

    /// Is pendant vertex.
    ///
    /// Checks whether the vertex is adjacent to only one vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<u32>> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is pendant (a.k.a is connected to just one vertex).
    /// assert_true!(g.is_pendant_vertex(&0)?);
    ///
    /// // Checking a non-existing vertex yields an error.
    /// assert_true!(g.is_pendant_vertex(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        Ok(self.degree_of(x)? == 1)
    }
}

/// Vertex iterator.
///
/// Return the vertex iterator representing $V(G)$.
///
#[macro_export]
macro_rules! V {
    ($g:expr) => {
        $g.vertex_iter()
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
        $g.adjacent_iter($x)
    };
}
