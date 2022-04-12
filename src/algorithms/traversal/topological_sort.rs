use std::{
    collections::{HashMap, VecDeque},
    iter::FusedIterator,
};

use crate::{traits::Directed, Ch, V};

/// Topological sort search structure.
pub struct TopologicalSort<'a, G>
where
    G: Directed,
{
    /// Given graph reference.
    graph: &'a G,
    // To-be-visited queue.
    queue: VecDeque<&'a G::Vertex>,
    // Visit map with vertices in-degrees.
    visit: HashMap<&'a G::Vertex, usize>,
}

impl<'a, G> TopologicalSort<'a, G>
where
    G: Directed,
{
    /// Build a TopologicalSort iterator.
    ///
    /// Build a TopologicalSort[^1] iterator for a given directed graph.
    ///
    /// If the graph is cyclic, this iterator returns an error while unrolling.
    ///
    /// [^1]: [Kahn, A. B. (1962). Topological sorting of large networks. Communications of the ACM, 5(11), 558-562.](https://scholar.google.com/scholar?q=Topological+sorting+of+large+networks)
    ///
    pub fn new(g: &'a G) -> Self {
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
        for x in V!(search.graph) {
            // Compute its in-degree.
            match search.graph.in_degree(x) {
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

impl<'a, G> Iterator for TopologicalSort<'a, G>
where
    G: Directed,
{
    type Item = Result<&'a G::Vertex, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        // While there are still vertices with zero in-degree.
        if let Some(x) = self.queue.pop_front() {
            // For each child of the selected vertex.
            for y in Ch!(self.graph, x) {
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

impl<'a, G> FusedIterator for TopologicalSort<'a, G> where G: Directed {}

impl<'a, G> From<&'a G> for TopologicalSort<'a, G>
where
    G: Directed,
{
    /// Builds a search object from a given graph.
    ///
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}
