mod adjacency_list;
mod graph_trait;

pub use adjacency_list::UndirectedAdjacencyListGraph;
pub use graph_trait::GraphTrait;

/// Default undirected graph implementation based on adjacency list.
pub type Graph = UndirectedAdjacencyListGraph<u32>;
