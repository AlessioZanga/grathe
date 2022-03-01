use super::Traversal;
use crate::traits::Storage;
use crate::types::VertexIterator;
use crate::V;
use std::collections::{HashMap, VecDeque};
use std::iter::FusedIterator;
use std::vec::Vec;

/// Depth-first search structure.
///
/// This structure contains the `discovery_time`, `finish_time` and `predecessor` maps.
///
pub struct DepthFirstSearch<'a, T>
where
    T: Storage,
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
    T: Storage,
{
    /// Build a DFS iterator.
    ///
    /// Build a DFS iterator for a given graph. This will execute the [`Tree`](super::Traversal)
    /// variant of the algorithm, if not specified otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the (optional) source vertex is not in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a directed graph.
    /// let g = DiGraph::from_edges([
    ///     (0, 1), (1, 2), (0, 3), (0, 4), (4, 5)
    /// ]);
    ///
    /// // Build the search object over said graph with `0` as source vertex.
    /// let mut search = DFS::from((&g, &0));
    ///
    /// // Collect visit order, preserving search structure.
    /// let order: Vec<_> = search.by_ref().collect();
    /// // The visit returns a pre-order.
    /// assert_eq!(order, [&0, &1, &2, &3, &4, &5]);
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
        x: Option<&'a T::Vertex>,
        f: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
        m: Traversal,
    ) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Set reachability function.
            reachable: f,
            // Initialize the to-be-visited queue with the source vertex.
            stack: Default::default(),
            // Initialize the global clock.
            time: Default::default(),
            // Initialize the discovery-time map.
            discovery_time: Default::default(),
            // Initialize the finish-time map.
            finish_time: Default::default(),
            // Initialize the predecessor map.
            predecessor: Default::default(),
        };
        // If the graph is null.
        if g.order() == 0 {
            // Assert source vertex is none.
            assert!(x.is_none());
            // Then, return the default search object.
            return search;
        }
        // Get source vertex, if any.
        let x = match x {
            // If no source vertex is given, choose the first one in the vertex set.
            None => V!(g).next().unwrap(),
            // Otherwise ...
            Some(x) => {
                // ... assert that source vertex is in graph.
                assert!(g.has_vertex(x));
                // Return given source vertex.
                x
            }
        };
        // If visit variant is Forest.
        if matches!(m, Traversal::Forest) {
            // Add vertices to the visit stack in reverse to preserve order.
            let mut queue = VecDeque::with_capacity(g.order());
            queue.extend(V!(g).filter(|&y| y != x));
            search.stack.extend(queue.iter().rev());
        }
        // Push source vertex onto the stack.
        search.stack.push(x);
        // Return search object.
        search
    }
}

impl<'a, T> Iterator for DepthFirstSearch<'a, T>
where
    T: Storage,
{
    type Item = &'a T::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&x) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if !self.discovery_time.contains_key(x) {
                // Set its discover time (as GRAY).
                self.discovery_time.insert(x, self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for y in (self.reachable)(self.graph, x) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(y) {
                        // Set predecessor.
                        self.predecessor.insert(y, x);
                        // Add to queue.
                        queue.push_front(y);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Return vertex in pre-order.
                return Some(x);
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if it is GRAY (not BLACK).
                if !self.finish_time.contains_key(x) {
                    // Set its finish time (as BLACK).
                    self.finish_time.insert(x, self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
        }

        None
    }
}

impl<'a, T> FusedIterator for DepthFirstSearch<'a, T> where T: Storage {}
