use crate::graphs::GraphTrait;
use crate::types::*;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

/// Depth-first search structure.
///
/// This structure contains the `discovery_time`, `finish_time` and `predecessor` maps.
/// The underlying algorithm implements a depth-first search *tree* procedure,
/// where all-and-only the vertices reachable form a given source vertex are visited.
///
pub struct DepthFirstSearch<'a, T>
where
    T: GraphTrait,
{
    /// Given graph reference.
    graph: &'a T,
    /// Reachable vertices of distance one from given vertex.
    reachable: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    /// The visit stack.
    stack: Vec<&'a T::Vertex>,
    /// Global time counter.
    pub time: usize,
    /// Discovery time of each discovered vertex.
    pub discovery_time: HashMap<&'a T::Vertex, usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: HashMap<&'a T::Vertex, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T::Vertex, &'a T::Vertex>,
}

impl<'a, T> DepthFirstSearch<'a, T>
where
    T: GraphTrait,
{
    /// Run DFS *tree* for a given directed graph.
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
    /// // Run DFS over said graph with `0` as source vertex.
    /// let mut search = DFS::from((&g, &0));
    ///
    /// // Consume the iterator in-place and assert later.
    /// while let Some(_) = search.next() {}
    ///
    /// // The source vertex has discovery-time equals to zero ...
    /// assert_eq!(search.discovery_time[&0], 0);
    /// // ... finish-time equals to two times the number of discovered vertices minus one ...
    /// assert_eq!(search.finish_time[&0], 2 * search.discovery_time.len() - 1);
    /// // ... and no predecessor by definition.
    /// assert_eq!(search.predecessor.contains_key(&0), false);
    ///
    /// // For example, vertex `5` has discovery-time equals to eight ...
    /// assert_eq!(search.discovery_time[&5], 8);
    /// // ... finish-time equals to nine ...
    /// assert_eq!(search.finish_time[&5], 9);
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
            stack: From::from([x]),
            // Initialize the global clock.
            time: 0,
            // Initialize the discovery-time map.
            discovery_time: Default::default(),
            // Initialize the finish-time map.
            finish_time: Default::default(),
            // Initialize the predecessor map.
            predecessor: Default::default(),
        }
    }
}

impl<'a, T> Iterator for DepthFirstSearch<'a, T>
where
    T: GraphTrait,
{
    type Item = &'a T::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        if let Some(&y) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if !self.discovery_time.contains_key(y) {
                // Set its discover time (as GRAY).
                self.discovery_time.insert(y, self.time);
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for z in (self.reachable)(self.graph, y) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(z) {
                        // Set predecessor.
                        self.predecessor.insert(z, y);
                        // Add to queue.
                        queue.push_back(z);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue.iter().rev());
                // Increment time.
                self.time += 1;
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if it is GRAY (not BLACK).
                if !self.finish_time.contains_key(y) {
                    // Set its finish time (as BLACK).
                    self.finish_time.insert(y, self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
            // Return next vertex.
            return Some(&y);
        }

        // Otherwise end is reached.
        None
    }
}
