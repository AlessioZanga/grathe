/// Traversal enumerator.
pub enum Traversal {
    /// Tree variant, i.e. only vertices reachable from the source vertex are visited.
    Tree,
    /// Forest variant, i.e. any vertex in the graph is visited.
    Forest,
}

mod breadth_first_search;
pub use breadth_first_search::BreadthFirstSearch;

/// Alias for breadth-first search.
pub type BFS<'a, G, D> = BreadthFirstSearch<'a, G, D>;

mod depth_first_search;
pub use depth_first_search::DepthFirstSearch;

/// Alias for depth-first search.
pub type DFS<'a, G, D> = DepthFirstSearch<'a, G, D>;

mod depth_first_search_edges;
pub use depth_first_search_edges::{DFSEdge, DepthFirstSearchEdges};

/// Alias for depth-first search.
pub type DFSEdges<'a, G, D> = DepthFirstSearchEdges<'a, G, D>;

mod lexicographic_breadth_first_search;
pub use lexicographic_breadth_first_search::LexicographicBreadthFirstSearch;

/// Alias for lexicographic breadth-first search.
pub type LexBFS<'a, G> = LexicographicBreadthFirstSearch<'a, G>;

mod lexicographic_depth_first_search;
pub use lexicographic_depth_first_search::LexicographicDepthFirstSearch;

/// Alias for lexicographic depth-first search.
pub type LexDFS<'a, G> = LexicographicDepthFirstSearch<'a, G>;

mod topological_sort;
pub use topological_sort::TopologicalSort;
