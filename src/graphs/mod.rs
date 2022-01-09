mod adjacency_list;

pub use adjacency_list::{DirectedAdjacencyListGraph, UndirectedAdjacencyListGraph};

/// Default undirected graph implementation based on adjacency list.
pub type Graph = UndirectedAdjacencyListGraph<i32>;

/// Default undirected graph with labels implementation based on adjacency list.
pub type Graphl = UndirectedAdjacencyListGraph<String>;

/// Default directed graph implementation based on adjacency list.
pub type DiGraph = DirectedAdjacencyListGraph<i32>;

/// Default directed graph with labels implementation based on adjacency list.
pub type DiGraphl = DirectedAdjacencyListGraph<String>;
