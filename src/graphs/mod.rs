mod adjacency_list;
mod graph_trait;

pub use adjacency_list::{DirectedAdjacencyListGraph, UndirectedAdjacencyListGraph};
pub use graph_trait::GraphTrait;

/// Default undirected graph implementation based on adjacency list.
pub type Graph = UndirectedAdjacencyListGraph<i32>;

/// Default undirected graph with labels implementation based on adjacency list.
pub type Graphl = UndirectedAdjacencyListGraph<String>;

/// Default directed graph implementation based on adjacency list.
pub type DiGraph = DirectedAdjacencyListGraph<i32>;

/// Default directed graph with labels implementation based on adjacency list.
pub type DiGraphl = DirectedAdjacencyListGraph<String>;
