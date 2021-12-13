use crate::errors::Error;
use crate::types::*;
use crate::{Adj, E, V};
use nasparse::CooMatrix;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

/// The base graph storage trait.
pub trait StorageTrait: Eq + PartialOrd + Default + Debug {
    /// Vertex identifier type.
    // TODO: Change FromPrimitive to Step once stable, use combination of x = T::default()
    // and x = Step::forward(x, 1) to increase Vertex in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type Vertex: VertexTrait;

    // TODO: Uncomment once associated type defaults are stable.
    // Edge identifier type.
    // type Edge = (&Self::Vertex, &Self::Vertex);

    /// Underlying storage type.
    type Storage;

    /// Base constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The base constructor of $\mathcal{G}$
    /// returns a null graph $G$ (i.x. both $V$ and $E$ are empty).
    ///
    /// # Examples
    ///
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

    /// Returns the capacity.
    ///
    /// Returns the number of vertex the graph can hold.
    /// Depending on the underlying storage, this could avoid reallocations.
    ///
    /// # Examples
    ///
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
    ///
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Reserve additional capacity.
    /// g.reserve(3);
    /// assert_ge!(g.capacity(), 3);
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    /// assert_ge!(g.capacity(), 100);
    ///
    /// // Shrink capacity to given minimum.
    /// g.shrink_to(50);
    /// assert_ge!(g.capacity(), 50);
    /// ```
    ///
    fn shrink_to(&mut self, min_capacity: usize);

    /// Shrinks the capacity.
    ///
    /// Shrinks the capacity of the graph as much as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build graph with given capacity.
    /// let mut g = Graph::with_capacity(100);
    /// assert_ge!(g.capacity(), 100);
    ///
    /// // Shrink capacity as much as possible.
    /// g.shrink_to_fit();
    /// assert_eq!(g.capacity(), 0);
    /// ```
    ///
    fn shrink_to_fit(&mut self);

    /// From order constructor.
    ///
    /// Construct a graph of a given order.
    ///
    /// # Examples
    ///
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
        let mut g = Self::with_capacity(order);
        for x in (0..order).map(|x| Self::Vertex::from_str(&x.to_string()).ok().unwrap()) {
            g.add_vertex(&x).ok();
        }

        g
    }

    /// From vertex constructor.
    ///
    /// Construct a graph from a given sequence of vertex, ignoring repeated ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique vertex.
    /// let sequence = vec![0, 3, 1, 2];
    ///
    /// // Build a graph by consuming a vector of vertex.
    /// let g = Graph::from_vertices(sequence);
    ///
    /// // Build a graph by consuming any `IntoIterator`.
    /// let h = Graph::from_vertices((0..4));
    ///
    /// assert_eq!(g, h);
    ///
    /// // A sequence of unique vertex.
    /// let sequence = vec!["0", "3", "1", "2"];
    ///
    /// // Build a graph by consuming a vector of vertex labels.
    /// let g = GraphWithLabels::from_vertices(sequence);
    /// ```
    ///
    fn from_vertices<'a, I>(iter: I) -> Self
    where
        Self: 'a,
        I: IntoIterator<Item = &'a Self::Vertex>,
    {
        // Get vertex iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Build graph with initial capacity.
        let mut g = Self::with_capacity(lower);
        // Add vertex to the graph.
        for x in iter {
            g.add_vertex(x).ok();
        }

        g
    }

    /// From edges constructor.
    ///
    /// Construct a graph from a given sequence of edges, ignoring repeated ones.
    ///
    /// # Examples
    ///
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
    /// // A sequence of unique vertex labels pairs.
    /// let sequence = vec![("0", "1"), ("2", "3"), ("1", "2")];
    ///
    /// // Build a graph by consuming a vector of vertex labels pairs.
    /// let g = GraphWithLabels::from_edges(sequence);
    /// assert_eq!(E!(g).collect::<Vec<_>>(), [("0", "1"), ("1", "2"), ("2", "3")]);
    /// ```
    ///
    fn from_edges<'a, I>(iter: I) -> Self
    where
        Self: 'a,
        I: IntoIterator<Item = (&'a Self::Vertex, &'a Self::Vertex)>,
    {
        // Get edges iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Build graph with initial capacity,
        // assuming average frequency of new vertex.
        let mut g = Self::with_capacity(lower);
        // Add edges to the graph.
        for (x, y) in iter {
            g.add_vertex(x).ok();
            g.add_vertex(y).ok();
            g.add_edge((x, y)).ok();
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of uniques edges.
    /// let x = EdgeList::from([(0, 1), (1, 0)]);
    ///
    /// // Build a graph from a sequence of edges.
    /// let g = Graph::from_edges(x.clone());
    ///
    /// // Return an edge list (a.k.a. a *set* of edges) from the graph.
    /// assert_eq!(g.to_edge_list(), x);
    /// ```
    ///
    fn to_edge_list(&self) -> EdgeList<Self::Vertex> {
        let mut out = EdgeList::new();
        for (x, y) in E!(self) {
            out.insert((x.clone(), y.clone()));
        }
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
            out.entry(x.clone()).or_default().insert(y.clone());
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build a 3rd order graph.
    /// let g = Graph::from_order(3);
    ///
    /// // Use the vertex set iterator.
    /// let V: Vec<_> = g.vertices_iter().collect();
    /// assert_eq!(V, [0, 1, 2]);
    ///
    /// // Use the associated macro 'V!'.
    /// assert_true!(g.vertices_iter().eq(V!(g)));
    ///
    /// // Iterate over the vertex set.
    /// for x in V!(g) {
    ///     assert_true!(g.has_vertex(&x));
    /// }
    /// ```
    ///
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<&'a Self::Vertex> + 'a>;

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
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a 3rd order graph.
    /// let g = Graph::from_edges([(0, 1), (1, 0)]);
    ///
    /// // Use the vertex set iterator.
    /// let x: Vec<_> = g.edges_iter().collect();
    /// assert_eq!(x, [(0, 1), (1, 0)]);
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
    fn edges_iter<'a>(&'a self)
        -> Box<dyn EdgeIterator<(&'a Self::Vertex, &'a Self::Vertex)> + 'a>;

    /// Adjacent iterator.
    ///
    /// Iterates over the adjacent vertex set $Adj(G, X)$ of a given vertex $E$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>>
    /// # {
    /// // Build a graph from edges.
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// let A: Vec<_> = g.adjacents_iter(&0)?.collect();
    /// assert_eq!(A, [0, 1, 2]);
    ///
    /// // Use the associated macro 'Adj!'.
    /// assert_true!(g.adjacents_iter(&0)?.eq(Adj!(g, &0)?));
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
    fn adjacents_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>;

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
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a 2nd order graph.
    /// let g = Graph::from_order(2);
    ///
    /// // Check vertex.
    /// assert_true!(g.has_vertex(&0));
    /// assert_false!(g.has_vertex(&2));
    ///
    /// // Build a null graph.
    /// let mut g = GraphWithLabels::default();
    ///
    /// // Add a vertex given its label.
    /// let i = g.add_vertex("0")?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_vertex("0"));
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
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex.
    /// let i = g.add_vertex(&0)?;
    /// assert_true!(g.has_vertex(&i));
    ///
    /// // Build a null graph.
    /// let mut g = GraphWithLabels::default();
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
    fn add_vertex<'a>(
        &mut self,
        x: &'a Self::Vertex,
    ) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;

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
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
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
    /// let mut g = GraphWithLabels::default();
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
    #[inline(always)]
    fn extend_vertices<'a, I>(&mut self, iter: I) -> Result<(), Error<Self::Vertex>>
    where
        Self: 'a,
        I: IntoIterator<Item = &'a Self::Vertex>,
    {
        // Get vertex iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add vertex to the graph.
        for x in iter {
            self.add_vertex(x)?;
        }

        Ok(())
    }

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
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
    fn del_vertex<'a>(
        &mut self,
        x: &'a Self::Vertex,
    ) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a graph.
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Check edge.
    /// assert_true!(g.has_edge(&(0, 1))?);
    ///
    /// // Checking an edge with non-existing vertex yields an error.
    /// assert_true!(g.has_edge(&(0, 42)).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = GraphWithLabels::default();
    ///
    /// // Add a edge given its label.
    /// let x = g.reserve_edge(("0", "1"))?;
    ///
    /// // Check that the newly added label is indeed in the graph.
    /// assert_true!(g.has_edge(("0", "1"));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn has_edge<'a>(
        &self,
        x: (&'a Self::Vertex, &'a Self::Vertex),
    ) -> Result<bool, Error<Self::Vertex>>;

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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a 2nd order graph.
    /// let mut g = Graph::from_order(2);
    ///
    /// // Add a new edge from vertex.
    /// let x = g.add_edge(&(0, 1))?;
    /// assert_true!(g.has_edge(&x)?);
    ///
    /// // Adding an existing edge yields an error.
    /// assert_true!(g.add_edge(&x).is_err());
    ///
    /// // Build a 3rd order graph.
    /// let mut g = GraphWithLabels::from_order(3);
    ///
    /// // Add a edge given its label.
    /// let x = g.add_edge(("0", "1"))?;
    ///
    /// // Adding an existing edge label yields an error.
    /// assert_true!(g.add_edge(("0", "1")).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn add_edge<'a>(
        &mut self,
        x: (&'a Self::Vertex, &'a Self::Vertex),
    ) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>>;

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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge with specific identifiers.
    /// let x = g.reserve_edge(&(0, 1))?;
    /// assert_true!(
    ///     g.has_vertex(&0) &&
    ///     g.has_vertex(&1) &&
    ///     g.has_edge(&x)?
    /// );
    ///
    /// // Reserving an existing edge yields an error.
    /// assert_true!(g.reserve_edge(&x).is_err());
    ///
    /// // Also, reserving a non-existing edge with.
    /// // at least one existing vertex yields an error...
    /// assert_true!(g.reserve_edge(&(0, 2)).is_err());
    ///
    /// // ..but adding them is fine.
    /// g.add_vertex(&2)?;
    /// assert_false!(g.add_edge(&(0, 2)).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = GraphWithLabels::default();
    ///
    /// // Reserve a edge given its identifier and label.
    /// let x = g.reserve_edge(("0", "1"))?;
    /// assert_true!(g.has_edge(&x)?);
    ///
    /// // Reserving an existing edge identifier or label yields an error.
    /// assert_true!(g.reserve_edge(("0", "1")).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn reserve_edge<'a>(
        &mut self,
        x: (&'a Self::Vertex, &'a Self::Vertex),
    ) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>> {
        self.add_vertex(&x.0)?;
        self.add_vertex(&x.1)?;
        self.add_edge(x)
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
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Extend graph with edges.
    /// g.extend_edges([(0, 3), (1, 2)])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edges([(0, 3)]).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[inline(always)]
    fn extend_edges<'a, I>(&mut self, iter: I) -> Result<(), Error<Self::Vertex>>
    where
        Self: 'a,
        I: IntoIterator<Item = (&'a Self::Vertex, &'a Self::Vertex)>,
    {
        // Get edge iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add edge to the graph.
        for x in iter {
            self.reserve_edge(x)?;
        }

        Ok(())
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
    /// // Build a null graph.
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge.
    /// let x = g.reserve_edge(&(0, 1))?;
    /// assert_true!(g.has_edge(&x)?);
    ///
    /// // Delete the newly added edge.
    /// let f = g.del_edge(&x)?;
    /// assert_false!(g.has_edge(&x)?);
    ///
    /// // Deleting a edge returns its identifier.
    /// assert_eq!(x, f);
    ///
    /// // Deleting a non-existing edge yields an error.
    /// assert_true!(g.del_edge(&x).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = GraphWithLabels::default();
    ///
    /// // Extend graph with edges.
    /// g.extend_edge([("0", "3"), ("1", "2")])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edge([("0", "3")]).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn del_edge<'a>(
        &mut self,
        x: (&'a Self::Vertex, &'a Self::Vertex),
    ) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>>;

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
    ///
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
    /// Degree of given vertex identifier $E$ as $|Adj(G, X)|$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
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
    fn degree_of<'a>(&self, x: &'a Self::Vertex) -> Result<usize, Error<Self::Vertex>> {
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
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
    fn is_isolated_vertex<'a>(&self, x: &'a Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
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
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), grathe::errors::Error<i32>> {
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
    fn is_pendant_vertex<'a>(&self, x: &'a Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
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
        $g.vertices_iter()
    };
}

/// Edge iterator.
///
/// Return the edges iterator representing $x(G)$.
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
