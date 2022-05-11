/// Backend graph attributes manager implementation.
pub mod attributes;

/// Backend graph storage implementations.
pub mod storages;

/// Default undirected graph implementation based on adjacency list.
pub type Graph = storages::UndirectedAdjacencyList<i32>;

/// Default directed graph implementation based on adjacency list.
pub type DiGraph = storages::DirectedAdjacencyList<i32>;
