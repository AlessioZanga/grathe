use crate::errors::VertexError;
use crate::types::*;
use crate::{E, V};
use nasparse::CooMatrix;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fmt::Debug;

/// The base graph trait.
pub trait GraphTrait: Eq + PartialOrd + Debug + Default {
    /// Vertex identifier type.
    // TODO: Change FromPrimitive to Step once stable, use combination of x = T::default()
    // and x = Step::forward(x, 1) to increase Vertex in from(order) constructor,
    // rather than constructing it from usize using FromPrimitive.
    type Vertex: VertexTrait;

    // TODO: Uncomment once associated type defaults are stable.
    // Edge identifier type.
    // type Edge = (Self::Vertex, Self::Vertex);

    /// Storage type.
    type Storage;

    /// Base constructor.
    ///
    /// Returns a null graph.
    ///
    fn new() -> Self;

    /// From order constructor.
    ///
    /// Construct a graph of a given order.
    ///
    fn from_order(order: usize) -> Self {
        Self::from_vertices((0..order).map(|x| Self::Vertex::from_usize(x).unwrap()))
    }

    /// From vertices constructor.
    ///
    /// Construct a graph from a given sequence of vertices.
    ///
    /// # Panics
    ///
    /// The vertex identifiers are not unique.
    ///
    fn from_vertices<Iter>(vertices: Iter) -> Self
    where
        Iter: IntoIterator<Item = Self::Vertex>,
    {
        let mut g = Self::new();
        for x in vertices {
            g.add_vertex(&x)
        }
        g
    }

    /// From edges constructor.
    ///
    /// Construct a graph from a given sequence of edges.
    ///
    /// # Panics
    ///
    /// The edge identifiers are not unique.
    ///
    fn from_edges<Iter>(edges: Iter) -> Self
    where
        Iter: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        let mut g = Self::new();
        for (x, y) in edges {
            g.try_add_vertex(&x).ok();
            g.try_add_vertex(&y).ok();
            g.add_edge(&(x, y));
        }
        g
    }

    /// Vertex iterator.
    ///
    /// Iterates over the vertex set $V$ ordered by identifier value.
    ///
    fn vertices_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a>;

    /// Edge iterator.
    ///
    /// Iterates over the edge set $E$ order by identifier values.
    ///
    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Vertex, Self::Vertex)> + 'a>;

    /// Data storage.
    ///
    /// Return immutable reference to internal data storage.
    ///
    fn as_data(&self) -> &Self::Storage;

    /// Edge list adapter.
    ///
    /// Return the edge list representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|E| \cdot \log |E|)$ - Log-linear in the size of the graph.
    ///
    fn as_edge_list(&self) -> EdgeList<Self::Vertex> {
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
    fn as_adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
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
    fn as_dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix {
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
    fn as_sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix {
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

    /// Order of the graph.
    ///
    /// Return the graph order (aka. $|V|$).
    ///
    /// # Complexity
    ///
    /// $O(1)$ - Constant.
    ///
    fn order(&self) -> usize;

    /// Size of the graph.
    ///
    /// Return the graph size (aka. $|E|$).
    ///
    fn size(&self) -> usize;

    /// Checks vertex in the graph.
    ///
    /// Checks whether the graph has a given vertex or not.
    ///
    fn has_vertex(&self, x: &Self::Vertex) -> bool;

    /// Adds vertex to the graph
    ///
    /// Insert given vertex identifier into the graph.
    ///
    /// # Panics
    ///
    /// The vertex identifier already exists in the graph.
    ///
    fn add_vertex(&mut self, x: &Self::Vertex) -> () {
        return self.try_add_vertex(x).unwrap();
    }

    /// Deletes vertex from the graph
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Panics
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    fn del_vertex(&mut self, x: &Self::Vertex) -> () {
        return self.try_del_vertex(x).unwrap();
    }

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> bool {
        return self.try_has_edge(e).unwrap();
    }

    /// Adds edge to the graph.
    ///
    /// Insert given edge identifier into the graph.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    fn add_edge(&mut self, e: &(Self::Vertex, Self::Vertex)) -> () {
        return self.try_add_edge(e).unwrap();
    }

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Panics
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier does not exists in the graph.
    ///
    fn del_edge(&mut self, e: &(Self::Vertex, Self::Vertex)) -> () {
        return self.try_del_edge(e).unwrap();
    }

    /// Adds vertex to the graph
    ///
    /// Insert given vertex identifier into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier already exists in the graph.
    ///
    fn try_add_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError>;

    /// Deletes vertex from the graph
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
    fn try_del_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError>;

    /// Checks edge in the graph.
    ///
    /// Checks whether the graph has a given edge or not.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph.
    ///
    fn try_has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, VertexError>;

    /// Adds edge to the graph.
    ///
    /// Insert given edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier already exists in the graph.
    ///
    fn try_add_edge(&mut self, e: &(Self::Vertex, Self::Vertex)) -> Result<(), VertexError>;

    /// Deletes edge from the graph.
    ///
    /// Remove given edge identifier from the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the edge identifier does not exists in the graph.
    ///
    fn try_del_edge(&mut self, e: &(Self::Vertex, Self::Vertex)) -> Result<(), VertexError>;
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
