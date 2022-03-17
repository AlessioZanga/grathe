/// Backend graph attributes manager implementation.
pub mod attributes;

/// Backend graph storage implementations.
pub mod storages;

/// Default undirected graph implementation based on adjacency list.
pub type Graph = storages::UndirectedAdjacencyList<i32>;

/// Default undirected graph with labels implementation based on adjacency list.
pub type Graphl = storages::UndirectedAdjacencyList<String>;

/// Default directed graph implementation based on adjacency list.
pub type DiGraph = storages::DirectedAdjacencyList<i32>;

/// Default directed graph with labels implementation based on adjacency list.
pub type DiGraphl = storages::DirectedAdjacencyList<String>;
