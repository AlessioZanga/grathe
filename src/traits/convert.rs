use ndarray::prelude::*;
use sprs::TriMat;

use crate::{
    traits::Storage,
    types::{AdjacencyList, DenseAdjacencyMatrix, DenseMarkMatrix, EdgeList, SparseAdjacencyMatrix, SparseMarkMatrix},
    Adj, E, V,
};

/// The graph conversion trait.
pub trait Convert: Storage {
    /// Edge list adapter.
    ///
    /// Return the edge list representing the graph.
    ///
    fn edge_list(&self) -> EdgeList<Self::Vertex> {
        E!(self).map(|(x, y)| (*x, *y)).collect()
    }

    /// Adjacency list adapter.
    ///
    /// Return the adjacency list representing the graph.
    ///
    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex> {
        V!(self)
            .map(|x| (*x, FromIterator::from_iter(Adj!(self, x).cloned())))
            .collect()
    }

    /// Dense adjacency matrix of a graph.
    ///
    /// The adjacency matrix $\textbf{A}$ of a graph $G$ is defined as:
    ///
    /// $$ \textbf{A}_{i,j} = \begin{cases} 1, & \text{if } (i, j) \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
    ///
    fn dense_adjacency_matrix(&self) -> DenseAdjacencyMatrix;

    /// Sparse adjacency matrix of a graph.
    ///
    /// Defined as its [dense variant][`Convert::dense_adjacency_matrix`].
    ///
    fn sparse_adjacency_matrix(&self) -> SparseAdjacencyMatrix;

    /// Dense incidence matrix of a graph.
    ///
    /// The incidence matrix $\textbf{B}$ of a graph $G$ is defined as:
    ///
    /// $$ \textbf{B}_{i,j} = \begin{cases} -1, & \text{if } (i, \cdot) \equiv e_j \in \textbf{E}, \newline +1, & \text{if } (\cdot, i) \equiv e_j \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
    ///
    fn dense_incidence_matrix(&self) -> Array2<i8>;

    /// Sparse incidence matrix of a graph.
    ///
    /// Defined as its [dense variant][`Convert::dense_incidence_matrix`].
    ///
    fn sparse_incidence_matrix(&self) -> TriMat<i8>;

    /// Dense M matrix of a graph.
    fn dense_mark_matrix(&self) -> DenseMarkMatrix;

    /// Dense M matrix of a graph.
    fn sparse_mark_matrix(&self) -> SparseMarkMatrix;
}
