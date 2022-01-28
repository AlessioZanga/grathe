use crate::traits::Base;
use crate::types::VertexIterator;
use crate::V;
use std::collections::{HashMap, VecDeque};

/// Breadth-first search structure.
///
/// This structure contains the `distance` and `predecessor` maps.
/// The underlying algorithm implements a breadth-first search *tree* procedure,
/// where all-and-only the vertices reachable form a given source vertex are visited.
///
pub struct BreadthFirstSearch<'a, T>
where
    T: Base,
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
    T: Base,
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
    /// // Build a BFS iterator over graph with `0` as source vertex.
    /// let mut search = BFS::from((&g, &0));
    ///
    /// // Consume the iterator in-place and assert later.
    /// while let Some(_) = search.next() {}
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
        x: Option<&'a T::Vertex>,
        f: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    ) -> Self {
        //
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Set reachability function.
            reachable: f,
            // Initialize the to-be-visited queue with the source vertex.
            queue: Default::default(),
            // Initialize the distance map.
            distance: Default::default(),
            // Initialize the predecessor map.
            predecessor: Default::default(),
        };
        // Get source vertex, if any.
        let x = match x {
            // If no source vertex is given ...
            None => {
                // If the graph is null.
                if g.order() == 0 {
                    // Then, return the default search object.
                    return search;
                }
                // ... choose the first one in the vertex set as source.
                V!(g).next().unwrap()
            }
            // Otherwise ...
            Some(x) => {
                // ... assert that source vertex is in graph.
                assert!(g.has_vertex(x));
                // Return given source vertex.
                x
            }
        };
        // Push the source vertex into the queue.
        search.queue.push_front(x);
        // Set its distance to zero.
        search.distance.insert(x, 0);
        // Return search object.
        search
    }
}

impl<'a, T> Iterator for BreadthFirstSearch<'a, T>
where
    T: Base,
{
    type Item = &'a T::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        if let Some(y) = self.queue.pop_front() {
            // Iterate over the reachable vertices of the popped vertex.
            for z in (self.reachable)(self.graph, y) {
                // If the vertex has never seen before.
                if !self.distance.contains_key(z) {
                    // Compute the distance from its predecessor.
                    self.distance.insert(z, self.distance[y] + 1);
                    // Set its predecessor.
                    self.predecessor.insert(z, y);
                    // Push it into the to-be-visited queue.
                    self.queue.push_back(z);
                }
            }
            // Return next vertex.
            return Some(y);
        }

        // Otherwise end is reached.
        None
    }
}
