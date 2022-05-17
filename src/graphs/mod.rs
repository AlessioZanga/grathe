/// Backend graph attributes manager implementation.
pub mod attributes;

/// Backend graph storage implementations.
pub mod storages;

/// Default undirected graph implementation based on adjacency list.
pub type Graph = storages::UndirectedAdjacencyList<i32>;

/// Default directed graph implementation based on adjacency list.
pub type DiGraph = storages::DirectedAdjacencyList<i32>;

/// Default partially-directed graph implementation based on dense adjacency matrix.
pub type PartiallyDirectedGraph = storages::PartiallyDirectedDenseAdjacencyMatrix<i32>;

/// Default partially-mixed graph implementation based on dense adjacency matrix.
pub type PartiallyMixedGraph = storages::PartiallyMixedDenseAdjacencyMatrix<i32>;
