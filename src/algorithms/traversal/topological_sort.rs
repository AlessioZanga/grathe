use crate::traits::Directed;
use std::collections::{HashMap, VecDeque};
use std::iter::FusedIterator;

/// Topological sort search structure.
pub struct TopologicalSort<'a, T>
where
    T: Directed,
{
    /// Given graph reference.
    graph: &'a T,
    // To-be-visited queue.
    queue: VecDeque<&'a T::Vertex>,
    // Visit map with vertices in-degrees.
    visit: HashMap<&'a T::Vertex, usize>,
}

impl<'a, T> TopologicalSort<'a, T>
where
    T: Directed,
{
    /// Build a TopologicalSort iterator.
    ///
    /// Build a TopologicalSort[^1] iterator for a given directed graph.
    ///
    /// If the graph is cyclic, this iterator returns an error while unrolling.
    ///
    /// [^1]: [Kahn, A. B. (1962). Topological sorting of large networks. Communications of the ACM, 5(11), 558-562.](https://scholar.google.com/scholar?q=Topological+sorting+of+large+networks)
    ///
    pub fn new(g: &'a T) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Initialize the to-be-visited queue with the source vertex.
            queue: Default::default(),
            // Initialize the visit map.
            visit: Default::default(),
        };
        // For each vertex in the graph.
        for x in search.graph.vertices_iter() {
            // Compute its in-degree.
            match search.graph.in_degree_of(x) {
                // If the in-degree is zero, then add it to the queue.
                0 => search.queue.push_back(x),
                // Otherwise, add it to the map.
                y => {
                    search.visit.insert(x, y);
                }
            }
        }

        search
    }
}

impl<'a, T> Iterator for TopologicalSort<'a, T>
where
    T: Directed,
{
    type Item = Result<&'a T::Vertex, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        // While there are still vertices with zero in-degree.
        if let Some(x) = self.queue.pop_front() {
            // For each child of the selected vertex.
            for y in self.graph.children_iter(x) {
                // If it was not visited before.
                if let Some(z) = self.visit.get(y) {
                    // Update its in-degree.
                    match z - 1 {
                        // If the in-degree is zero ...
                        0 => {
                            // ... then add it to the queue ...
                            self.queue.push_back(y);
                            // ... and remove it from the visit map.
                            self.visit.remove(y);
                        }
                        // Otherwise, update its in-degree into the map.
                        z => {
                            self.visit.insert(y, z);
                        }
                    }
                }
            }
            // Return current vertex.
            return Some(Ok(x));
        }

        // If there are still vertices with non-zero in-degree ...
        if !self.visit.is_empty() {
            // ... no topological sort is defined, i.e. cyclic graph.
            return Some(Err(()));
        }

        None
    }
}

impl<'a, T> FusedIterator for TopologicalSort<'a, T> where T: Directed {}

impl<'a, T> From<&'a T> for TopologicalSort<'a, T>
where
    T: Directed,
{
    /// Builds a search object from a given graph.
    ///
    fn from(g: &'a T) -> Self {
        Self::new(g)
    }
}
