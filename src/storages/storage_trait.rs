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
    /// assert_eq!(g.order(), 3);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Vertex identifiers are added incrementally starting from zero.
    /// assert_true!(g.has_vertex(&2));
    /// ```
    ///
    fn from_order(order: usize) -> Self {
        Self::from_vertices((0..order).map(|x| Self::Vertex::from_usize(x).unwrap()))
    }

    /// From vertices constructor.
    ///
    /// Construct a graph from a given sequence of vertices, ignoring repeated ones.
    ///
    /// # Examples
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // A sequence of unique vertices
    /// let sequence = vec![0, 3, 1, 2];
    ///
    /// // Build a graph by consuming a vector of vertices
    /// let g = Graph::from_vertices(sequence);
    ///
    /// // Build a graph by consuming any `IntoIterator`
    /// let h = Graph::from_vertices((0..4));
    ///
    /// assert_eq!(g, h);
    ///
    /// ```
    ///
    fn from_vertices<Iter>(vertices: Iter) -> Self
    where
        Iter: IntoIterator<Item = Self::Vertex>,
    {
        let mut g = Self::new();
        for x in vertices {
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
    /// // A sequence of unique edges
    /// let sequence = vec![(0, 1), (2, 3), (1, 2)];
    ///
    /// // Build a graph by consuming a vector of edges
    /// let g = Graph::from_edges(sequence);
    ///
    /// // Build a graph by consuming any `IntoIterator`
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
        let mut g = Self::new();
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
    /// // A sequence of uniques edges
    /// let E = EdgeList::from([(0, 1), (1, 0)]);
    ///
    /// // Build a graph from a sequence of edges
    /// let g = Graph::from_edges(E.clone());
    ///
    /// // Return an edge list (a.k.a. a *set* of edges) from the graph
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
    /// // Build a 3rd order graph
    /// let g = Graph::from_order(3);
    ///
    /// // Use the vertex set iterator
    /// let V: Vec<_> = g.vertices_iter().collect();
    /// assert_eq!(V, [0, 1, 2]);
    ///
    /// // Use the associated macro 'V!'
    /// assert_true!(g.vertices_iter().eq(V!(g)));
    ///
    /// // Iterate over the vertex set
    /// for x in V!(g) {
    ///     assert_true!(g.has_vertex(&x));
    /// }
    /// ```
    ///
    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<Self::Vertex> + 'a>;

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
    /// // Build a 3rd order graph
    /// let g = Graph::from_edges([(0, 1), (1, 0)]);
    ///
    /// // Use the vertex set iterator
    /// let E: Vec<_> = g.edges_iter().collect();
    /// assert_eq!(E, [(0, 1), (1, 0)]);
    ///
    /// // Use the associated macro 'E!'
    /// assert_true!(g.edges_iter().eq(E!(g)));
    ///
    /// // Iterate over the vertex set
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
    /// Iterates over the adjacent vertices set $Adj(G, X)$ of a given vertex $X$.
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
    /// // Build a graph from edges
    /// let g = Graph::from_edges([(0, 1), (2, 0), (0, 0)]);
    ///
    /// // Use the adjacent iterator.
    /// let A: Vec<_> = g.adjacent_iter(&0)?.collect();
    /// assert_eq!(A, [0, 1, 2]);
    ///
    /// // Use the associated macro 'Adj!'
    /// assert_true!(g.adjacent_iter(&0)?.eq(Adj!(g, &0)?));
    ///
    /// // Iterate over the adjacent set
    /// for x in Adj!(g, &0)? {
    ///     assert_true!(g.has_edge(&(0, x))?);
    /// }
    ///
    /// // Iterating over non-existing vertex yields an error
    /// assert_true!(Adj!(g, &3).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn adjacent_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// Vertices labels.
    ///
    /// Return immutable reference to internal vertices labels.
    ///
    fn as_vertices_labels(&self) -> &LabelMap<Self::Vertex>;

    /// Mutable vertices labels.
    ///
    /// Return mutable reference to internal vertices labels.
    ///
    fn as_mut_vertices_labels(&mut self) -> &mut LabelMap<Self::Vertex>;

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
    /// // Build a 5th order graph
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
    /// // Build a 5th size graph
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
    /// // Build a 2nd order graph
    /// let g = Graph::from_order(2);
    ///
    /// // Check vertex
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex with a specific identifier
    /// let i = g.reserve_vertex(&42)?;
    /// assert_true!(g.has_vertex(&i) && i == 42);
    ///
    /// // Adding an existing vertex yields an error
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new vertex
    /// let i = g.add_vertex()?;
    /// assert_true!(g.has_vertex(&i));
    ///
    /// // Delete the newly added vertex
    /// let j = g.del_vertex(&i)?;
    /// assert_false!(g.has_vertex(&i));
    ///
    /// // Deleting a vertex returns its identifier
    /// assert_eq!(i, j);
    ///
    /// // Deleting a non-existing vertex yields an error
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
    /// // Build a graph
    /// let g = Graph::from_edges([(0, 1), (3, 2)]);
    ///
    /// // Check edge
    /// assert_true!(g.has_edge(&(0, 1))?);
    ///
    /// // Checking an edge with non-existing vertex yields an error
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge with vertices
    /// let i = g.add_vertex()?;
    /// let j = g.add_vertex()?;
    /// let e = g.add_edge(&(i, j))?;
    /// assert_true!(g.has_edge(&e)?);
    ///
    /// // Adding an existing edge yields an error
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge with specific identifiers
    /// let e = g.reserve_edge(&(0, 1))?;
    /// assert_true!(
    ///     g.has_vertex(&0) &&
    ///     g.has_vertex(&1) &&
    ///     g.has_edge(&e)?
    /// );
    ///
    /// // Reserving an existing edge yields an error
    /// assert_true!(g.reserve_edge(&e).is_err());
    ///
    /// // Also, reserving a non-existing edge with
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
    /// // Build a null graph
    /// let mut g = Graph::default();
    ///
    /// // Add a new edge
    /// let e = g.reserve_edge(&(0, 1))?;
    /// assert_true!(g.has_edge(&e)?);
    ///
    /// // Delete the newly added edge
    /// let f = g.del_edge(&e)?;
    /// assert_false!(g.has_edge(&e)?);
    ///
    /// // Deleting a edge returns its identifier
    /// assert_eq!(e, f);
    ///
    /// // Deleting a non-existing edge yields an error
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
    #[inline(always)]
    fn has_vertex_label(&self, x: &str) -> bool {
        self.as_vertices_labels().contains_right(x)
    }

    /// Adds vertex label to the graph.
    ///
    /// Insert given vertex label into the graph.
    ///
    /// # Errors
    ///
    /// The vertex label already exists in the graph.
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
    #[inline(always)]
    fn get_vertex_id(&self, x: &str) -> Result<Self::Vertex, Error<Self::Vertex>> {
        self.as_vertices_labels()
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
    #[inline(always)]
    fn get_vertex_label(&self, x: &Self::Vertex) -> Result<String, Error<Self::Vertex>> {
        self.as_vertices_labels()
            .get_by_left(x)
            .map(String::from)
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
    fn set_vertex_label(
        &mut self,
        x: &Self::Vertex,
        y: &str,
    ) -> Result<Self::Vertex, Error<Self::Vertex>> {
        match self.has_vertex(x) {
            false => Err(Error::VertexNotDefined(*x)),
            true => match self.as_vertices_labels().contains_right(y) {
                false => {
                    // Overwrite previous label if present
                    self.as_mut_vertices_labels().insert(*x, String::from(y));
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
    fn unset_vertex_label(
        &mut self,
        x: &Self::Vertex,
    ) -> Result<(Self::Vertex, String), Error<Self::Vertex>> {
        self.as_mut_vertices_labels()
            .remove_by_left(x)
            .ok_or(Error::VertexNotDefined(*x))
    }

    /// Checks edge label.
    ///
    /// Checks whether the graph has a given edge label or not.
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
    /// Insert given vertices identifiers, edge identifier and edge label into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers already exists in the graph,
    /// or the edge identifier or label already exists in the graph.
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
    #[inline(always)]
    fn get_edge_label(
        &self,
        x: &(Self::Vertex, Self::Vertex),
    ) -> Result<String, Error<Self::Vertex>> {
        self.as_edges_labels()
            .get_by_left(x)
            .map(String::from)
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
    #[inline(always)]
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Is supergraph of another graph.
    ///
    /// Checks if this graph is supergraph of given graph.
    ///
    #[inline(always)]
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }

    /// Clears the graph.
    ///
    /// Deletes the vertices and edges with the associated attributes.
    ///
    fn clear(&mut self);

    /// Degree of vertex.
    ///
    /// Degree of given vertex identifier $X$ as $|Adj(G, X)|$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
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
    #[inline(always)]
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        Ok(self.degree_of(x)? == 1)
    }
}

/// Vertex iterator.
///
/// Return the vertices iterator representing $V(G)$.
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
/// Return the vertices iterator representing $Adj(G, X)$.
///
#[macro_export]
macro_rules! Adj {
    ($g:expr, $x:expr) => {
        $g.adjacent_iter($x)
    };
}
