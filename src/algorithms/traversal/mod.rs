mod breadth_first_search;
pub use breadth_first_search::BreadthFirstSearch;

/// Alias for breadth-first search.
pub type BFS<'a, T> = BreadthFirstSearch<'a, T>;

mod depth_first_search;
pub use depth_first_search::DepthFirstSearch;

/// Alias for depth-first search.
pub type DFS<'a, T> = DepthFirstSearch<'a, T>;
