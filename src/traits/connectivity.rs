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
    fn is_connected(&self) -> bool {
        // If the graph is null the return true.
        if self.order() == 0 {
            return true;
        }
        // Otherwise select a random vertex as source vertex.
        let x = self.vertices_iter().next().unwrap();
        // Initialize the to-be-visited queue with the source vertex.
        let mut queue = std::collections::VecDeque::from([x]);
        // Initialize the visited set.
        let mut visited = std::collections::HashSet::from([x]);
        // If there are still vertices to be visited.
        while let Some(y) = queue.pop_front() {
            // Iterate over the adjacent vertices of the popped vertex.
            // This step allow to visit any graph irrespectively of their direction.
            for z in self.adjacents_iter(y) {
                // If the vertex was never seen before.
                if !visited.contains(z) {
                    // Set as visited.
                    visited.insert(z);
                    // Push it into the to-be-visited queue.
                    queue.push_back(z);
                }
            }
        }
        // If there are still vertices not visited, the graph is not connected.
        visited.len() == self.order()
    }

    /// Checks global acyclicity.
    ///
    /// Checks whether the graph contains at least one cycle.
    ///
    fn is_acyclic(&self) -> bool;
}
