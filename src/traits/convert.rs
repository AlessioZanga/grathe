use crate::traits::Storage;
use crate::types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, SparseAdjacencyMatrix};
use crate::{E, V};
use std::collections::HashMap;

/// The graph conversion trait.
pub trait Convert: Storage {
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
    /// let sequence = [(0, 1), (1, 0)];
    ///
    /// // Build a graph from a sequence of edges.
    /// let g = Graph::from_edges(sequence);
    ///
    /// // Return an edge list (a.k.a. a *set* of edges) from the graph.
    /// assert_eq!(g.edge_list(), EdgeList::from(sequence));
    /// ```
    ///
    fn edge_list(&self) -> EdgeList<Self::Vertex>;

    /// Adjacency list adapter.
    ///
    /// Return the adjacency list representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex>;

    /// Dense adjacency matrix adapter.
    ///
    /// Return the (dense) adjacency matrix representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V|^2)$ - Quadratic in the order of the graph.
    ///
    fn dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix;

    /// Sparse adjacency matrix adapter.
    ///
    /// Return the (sparse) adjacency matrix representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    fn sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix;
}

impl<T> Convert for T
where
    T: Storage,
{
    fn edge_list(&self) -> EdgeList<Self::Vertex> {
        let mut out = EdgeList::new();
        for (x, y) in E!(&self) {
            out.insert((x.clone(), y.clone()));
        }

        out
    }

    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
        let mut out = AdjacencyList::new();
        for (x, y) in E!(&self) {
            out.entry(x.clone()).or_default().insert(y.clone());
        }

        out
    }

    fn dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::new();
        let mut out = DenseAdjacencyMatrix::from_elem((n, n), false);
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(i, x)| (x, i)));
        // Populate the output value.
        for (x, y) in E!(&self) {
            out[(idx[&x], idx[&y])] = true;
        }

        out
    }

    fn sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix {
        let n = self.order();
        let mut idx = HashMap::new();
        let mut out = SparseAdjacencyMatrix::new((n, n));
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(index, vid)| (vid, index)));
        // Populate the output value.
        out.reserve(self.size());
        for (x, y) in E!(&self) {
            out.add_triplet(idx[&x], idx[&y], true);
        }

        out
    }
}
