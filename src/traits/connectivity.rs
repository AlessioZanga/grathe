use super::Storage;

/// The graph connectivity trait.
pub trait Connectivity: Storage {
    /// Checks path connectivity.
    ///
    /// Checks whether two vertices are connected by a path.
    ///
    /// # Panics
    ///
    /// Panics if at least one of the two vertices is not in the graph.
    ///
    fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Checks global connectivity.
    ///
    /// Checks whether the graph is connected.
    ///
    /// For directed graph this procedure is equivalent to check if the graph
    /// is *weakly connected*, that means testing whether the undirected graph
    /// obtained by replacing all the edges with undirected edges is connected.
    ///
    fn is_connected(&self) -> bool;

    /// Checks global acyclicity.
    ///
    /// Checks whether the graph contains at least one cycle.
    ///
    fn is_acyclic(&self) -> bool;
}
