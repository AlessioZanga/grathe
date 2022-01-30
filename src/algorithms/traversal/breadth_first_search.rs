use super::Traversal;
use crate::traits::Base;
use crate::types::VertexIterator;
use crate::V;
use std::collections::{HashMap, VecDeque};
use std::iter::FusedIterator;

/// Breadth-first search structure.
///
/// This structure contains the `distance` and `predecessor` maps.
///
/// If the algorithm is set to the [`Forest`](super::Traversal) variant, a vertex with distance
/// [`usize::MAX`] means that such vertex is not reachable from the given
/// source vertex (i.e. the graph is disconnected).
///
pub struct BreadthFirstSearch<'a, T>
where
    T: Base,
{
    /// Given graph reference.
    graph: &'a T,
    /// To-be-visited queue for the [`Forest`](super::Traversal) variant.
    vertices: VecDeque<&'a T::Vertex>,
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
    /// Build a BFS iterator for a given graph. This will execute the [`Tree`](super::Traversal)
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
        m: Traversal,
    ) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Initialize the [`Forest`] to-be-visited queue.
            vertices: Default::default(),
            // Set reachability function.
            reachable: f,
            // Initialize the to-be-visited queue with the source vertex.
            queue: Default::default(),
            // Initialize the distance map.
            distance: Default::default(),
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
        // If visit variant is [`Forest`] ...
        if matches!(m, Traversal::Forest) {
            // ... fill the vertices queue.
            search.vertices.extend(V!(g));
        }
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
        // If there are no more vertices in the queue ...
        if self.queue.is_empty() {
            // ... but there is still a vertex ...
            while let Some(x) = self.vertices.pop_front() {
                // ... that was not visited.
                if self.distance.contains_key(x) {
                    // Push such vertex into the visiting queue.
                    self.queue.push_front(x);
                    // Set its distance to usize::MAX, since it is
                    // not reachable from the original source vertex.
                    self.distance.insert(x, usize::MAX);
                }
            }
        }
        // If there are still vertices to be visited.
        if let Some(y) = self.queue.pop_front() {
            // Iterate over the reachable vertices of the popped vertex.
            for z in (self.reachable)(self.graph, y) {
                // If the vertex has never seen before.
                if !self.distance.contains_key(z) {
                    // Compute the distance from its predecessor.
                    // NOTE: This operation is implemented using a
                    // `saturating_add` in order to avoid overflowing in
                    // the [`Forest`] variant, where `infinity` is usize::MAX.
                    self.distance.insert(z, self.distance[y].saturating_add(1));
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

impl<'a, T> FusedIterator for BreadthFirstSearch<'a, T> where T: Base {}
