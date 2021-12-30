mod breadth_first_search;
mod depth_first_search;

pub use breadth_first_search::BreadthFirstSearch;
pub use depth_first_search::DepthFirstSearch;

/// Alias for breadth-first search.
pub type BFS<'a, T> = BreadthFirstSearch<'a, T>;

/// Alias for depth-first search.
pub type DFS<'a, T> = DepthFirstSearch<'a, T>;
