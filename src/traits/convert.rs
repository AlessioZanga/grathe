use crate::traits::Storage;
use crate::types::{AdjacencyList, EdgeList};
use ndarray::Array2;
use sprs::TriMat;

/// The graph conversion trait.
pub trait Convert: Storage {
    /// Edge list adapter.
    ///
    /// Return the edge list representing the graph.
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
    fn adjacency_list(&self) -> AdjacencyList<Self::Vertex>;

    /// Dense adjacency matrix of a graph.
    ///
    /// The adjacency matrix $\textbf{A}$ of a graph $G$ is defined as:
    ///
    /// $$ \textbf{A}_{i,j} = \begin{cases} 1, & \text{if } (i, j) \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
    ///
    fn dense_adjacency_matrix(&self) -> Array2<bool>;

    /// Sparse adjacency matrix of a graph.
    ///
    /// Defined as its [dense variant][`Convert::dense_adjacency_matrix`].
    ///
    fn sparse_adjacency_matrix(&self) -> TriMat<bool>;

    /// Dense incidence matrix of a graph.
    ///
    /// The incidence matrix $\textbf{B}$ of a graph $G$ is defined as:
    ///
    /// $$ \textbf{B}_{i,j} = \begin{cases} +1, & \text{if } (i, \cdot) \equiv e_j \in \textbf{E}, \newline -1, & \text{if } (\cdot, i) \equiv e_j \in \textbf{E}, \newline 0, & \text{Otherwise.} \end{cases} $$
    ///
    fn dense_incidence_matrix(&self) -> Array2<i8>;

    /// Sparse incidence matrix of a graph.
    ///
    /// Defined as its [dense variant][`Convert::dense_incidence_matrix`].
    ///
    fn sparse_incidence_matrix(&self) -> TriMat<i8>;
}
