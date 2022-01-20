use crate::traits::Storage;
use crate::types::VertexIterator;
use std::collections::{HashMap, VecDeque};

/// Breadth-first search structure.
///
/// This structure contains the `distance` and `predecessor` maps.
/// The underlying algorithm implements a breadth-first search *tree* procedure,
/// where all-and-only the vertices reachable form a given source vertex are visited.
///
pub struct BreadthFirstSearch<'a, T>
where
    T: Storage,
{
    /// Given graph reference.
    graph: &'a T,
    /// Reachable vertices of distance one from given vertex.
    reachable: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    /// To-be-visited queue with the source vertex.
    queue: VecDeque<&'a T::Vertex>,
    /// Distance from the source vertex.
    pub distance: HashMap<&'a T::Vertex, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T::Vertex, &'a T::Vertex>,
}

impl<'a, T> BreadthFirstSearch<'a, T>
where
    T: Storage,
{
    /// Build a new BFS iterator.
    ///
    /// Build a BFS *tree* iterator for a given graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a directed graph.
    /// let g = DiGraph::from_edges(&[
    ///     (0,  1), (1, 2), (0, 3), (0, 4), (4, 5)
    /// ]);
    ///
    /// // Build the search object over graph with `0` as source vertex.
    /// let mut search = BFS::from((&g, &0));
    ///
    /// // Consume the iterator in-place and assert later.
    /// search.run();
    ///
    /// // The source vertex has distance zero from itself ...
    /// assert_eq!(search.distance[&0], 0);
    /// // ... and no predecessor by definition.
    /// assert_eq!(search.predecessor.contains_key(&0), false);
    ///
    /// // For example, vertex `5` has distance two from `0` ...
    /// assert_eq!(search.distance[&5], 2);
    /// // ... and its predecessor is `4`.
    /// assert_eq!(search.predecessor[&5], &4);
    /// ```
    ///
    pub fn new(
        g: &'a T,
        x: &'a T::Vertex,
        f: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    ) -> Self {
        // Assert that source vertex is in graph.
        assert!(g.has_vertex(x));

        Self {
            // Set target graph.
            graph: g,
            // Set reachability function.
            reachable: f,
            // Initialize the to-be-visited queue with the source vertex.
            queue: From::from([x]),
            // Initialize the distance map.
            distance: From::from([(x, 0)]),
            // Initialize the predecessor map.
            predecessor: Default::default(),
        }
    }

    /// Execute the procedure.
    ///
    /// Execute the procedure and store the results for later queries.
    ///
    pub fn run(&mut self) -> &Self {
        while let Some(_) = self.next() {}

        self
    }
}

impl<'a, T> Iterator for BreadthFirstSearch<'a, T>
where
    T: Storage,
{
    type Item = &'a T::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        if let Some(x) = self.queue.pop_front() {
            // Iterate over the reachable vertices of the popped vertex.
            for y in (self.reachable)(self.graph, x) {
                // If the vertex was never seen before.
                if !self.distance.contains_key(y) {
                    // Compute the distance from its predecessor.
                    self.distance.insert(y, self.distance[x] + 1);
                    // Set its predecessor.
                    self.predecessor.insert(y, x);
                    // Push it into the to-be-visited queue.
                    self.queue.push_back(y);
                }
            }
            // Return next vertex.
            return Some(x);
        }

        // Otherwise end is reached.
        None
    }
}
