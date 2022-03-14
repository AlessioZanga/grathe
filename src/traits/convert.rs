use crate::traits::Storage;
use crate::types::{AdjacencyList, EdgeList};
use crate::{E, V};
use ndarray::Array2;
use sprs::TriMat;
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
    fn dense_adjacency_matrix(&self) -> Array2<bool>;

    /// Sparse adjacency matrix adapter.
    ///
    /// Return the (sparse) adjacency matrix representing the graph.
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    fn sparse_adjacency_matrix(&self) -> TriMat<bool>;

    fn dense_incidence_matrix(&self) -> Array2<bool>;

    fn sparse_incidence_matrix(&self) -> TriMat<bool>;
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

    fn dense_adjacency_matrix(&self) -> Array2<bool> {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = Array2::from_elem((n, n), false);
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(&self) {
            out[(idx[&x], idx[&y])] = true;
        }

        out
    }

    fn sparse_adjacency_matrix(&self) -> TriMat<bool> {
        let n = self.order();
        let mut idx = HashMap::with_capacity(n);
        let mut out = TriMat::new((n, n));
        // Reserve capacity for sparse matrix.
        out.reserve(self.size());
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (x, y) in E!(&self) {
            out.add_triplet(idx[&x], idx[&y], true);
        }

        out
    }

    fn dense_incidence_matrix(&self) -> Array2<bool> {
        let (n, m) = (self.order(), self.size());
        let mut idx = HashMap::with_capacity(n);
        let mut out = Array2::from_elem((n, m), false);
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in E!(&self).enumerate() {
            out[(idx[&x], i)] = true;
            out[(idx[&y], i)] = true;
        }

        out
    }

    fn sparse_incidence_matrix(&self) -> TriMat<bool> {
        let (n, m) = (self.order(), self.size());
        let mut idx = HashMap::with_capacity(n);
        let mut out = TriMat::new((n, m));
        // Reserve capacity for sparse matrix.
        out.reserve(2 * m);
        // Build vid-to-index mapping.
        idx.extend(V!(&self).enumerate().map(|(i, x)| (x, i)));
        // Fill the output matrix.
        for (i, (x, y)) in E!(&self).enumerate() {
            out.add_triplet(idx[&x], i, true);
            out.add_triplet(idx[&y], i, true);
        }

        out
    }
}
