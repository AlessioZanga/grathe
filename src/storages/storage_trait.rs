use crate::errors::*;
use crate::io::*;
use crate::types::*;
use crate::{Adj, E, V};
use nasparse::CooMatrix;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

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
    /// Panics if the vertex identifiers are not unique.
    ///
    fn from_vertices<Iter>(vertices: Iter) -> Self
    where
        Iter: IntoIterator<Item = Self::Vertex>,
    {
        let mut g = Self::new();
        for x in vertices {
            g.reserve_vertex(&x).unwrap();
        }
        g
    }

    /// From edges constructor.
    ///
    /// Construct a graph from a given sequence of edges.
    ///
    /// # Panics
    ///
    /// Panics if the edge identifiers are not unique.
    ///
    fn from_edges<Iter>(edges: Iter) -> Self
    where
        Iter: IntoIterator<Item = (Self::Vertex, Self::Vertex)>,
    {
        let mut g = Self::new();
        for (x, y) in edges {
            g.reserve_vertex(&x).ok();
            g.reserve_vertex(&y).ok();
            g.add_edge(&(x, y)).unwrap();
        }
        g
    }

    /// From DOT string constructor.
    ///
    /// Construct a graph from a given DOT string.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn from_dot(string: &str) -> Result<Self, Error<Self::Vertex>> {
        let mut g = from_dot::<Self>(string).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// Edge list adapter.
    ///
    /// Return the edge list representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|E| \cdot \log |E|)$ - Log-linear in the size of the graph.
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

    /// To DOT string.
    ///
    /// Return a DOT string representation of the graph.
    ///
    fn to_dot(&self) -> Result<String, Error<Self::Vertex>> {
        to_dot::<Self>(&self).map_err(|x| Error::ParseFailed(x.to_string()))
    }

    /// Read DOT file constructor.
    ///
    /// Construct a graph from a given DOT file.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn read_dot(path: &Path) -> Result<Self, Error<Self::Vertex>> {
        let mut g = read_dot::<Self>(path).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// Write DOT file.
    ///
    /// Write the graph to a given DOT file.
    ///
    fn write_dot(&self, path: &Path) -> Result<(), Error<Self::Vertex>> {
        write_dot::<Self>(path, &self).map_err(|x| Error::ParseFailed(x.to_string()))
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

    /// Adjacent iterator.
    ///
    /// Iterates over the adjacent vertices set $Adj(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exists in the graph.
    ///
    fn adjacents_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn Iterator<Item = Self::Vertex> + 'a>, Error<Self::Vertex>>;

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

    /// Adds vertex to the graph.
    ///
    /// Insert a new vertex identifier into the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier already exists in the graph.
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
    fn reserve_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>>;

    /// Deletes vertex from the graph.
    ///
    /// Remove given vertex identifier from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
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
    fn del_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;

    /// Checks vertex label.
    ///
    /// Checks whether the graph has a given vertex label or not.
    ///
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
    fn reserve_vertex_label(
        &mut self,
        x: &Self::Vertex,
        y: &str,
    ) -> Result<Self::Vertex, Error<Self::Vertex>> {
        self.reserve_vertex(x)?;
        self.set_vertex_label(&x, y)
    }

    /// Vertex identifier from label.
    ///
    /// Return vertex identifier given its label.
    ///
    /// # Errors
    ///
    /// The vertex label does not exists in the graph.
    ///
    fn get_vertex_id(&self, x: &str) -> Result<Self::Vertex, Error<Self::Vertex>> {
        self.as_vertices_labels()
            .get_by_right(x)
            .copied()
            .ok_or(Error::VertexLabelNotDefined(String::from(x)))
    }

    /// Vertex label from identifier.
    ///
    /// Return vertex label given its identifier.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exists in the graph.
    ///
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
    fn add_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.add_edge(x)?;
        self.set_edge_label(&x, y)
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
    fn reserve_edge_label(
        &mut self,
        x: &(Self::Vertex, Self::Vertex),
        y: &str,
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.reserve_edge(x)?;
        self.set_edge_label(&x, y)
    }

    /// Edge identifier from label.
    ///
    /// Return edge identifier given its label.
    ///
    /// # Errors
    ///
    /// The edge label does not exists in the graph.
    ///
    fn get_edge_id(&self, x: &str) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.as_edges_labels()
            .get_by_right(x)
            .copied()
            .ok_or(Error::EdgeLabelNotDefined(String::from(x)))
    }

    /// Edge label from identifier.
    ///
    /// Return edge label given its identifier.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exists in the graph.
    ///
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
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Is supergraph of another graph.
    ///
    /// Checks if this graph is supergraph of given graph.
    ///
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
        $g.adjacents_iter($x)
    };
}