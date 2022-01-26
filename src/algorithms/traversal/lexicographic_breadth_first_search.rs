use crate::traits::Undirected;
use std::collections::{HashSet, VecDeque};
use std::iter::FusedIterator;

/// Lexicographic breadth-first search structure.
pub struct LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Given graph reference.
    graph: &'a T,
    /// Current index.
    index: usize,
    /// To-be-visited queue.
    pub partitions: VecDeque<VecDeque<&'a T::Vertex>>,
}

impl<'a, T> LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Build a new LexBFS iterator.
    ///
    /// Build a LexBFS[^1] iterator for a given undirected graph.
    ///
    /// [^1]: [Bretscher, A., Corneil, D., Habib, M., & Paul, C. (2003, June). A simple linear time LexBFS cograph recognition algorithm.](https://scholar.google.com/scholar?q=+A+simple+linear+time+LexBFS+cograph+recognition+algorithm)
    ///
    pub fn new(g: &'a T) -> Self {
        Self {
            // Set target graph.
            graph: g,
            // Initialize index.
            index: Default::default(),
            // Initialize the to-be-visited queue with the first partition, if any.
            partitions: {
                // Cover the null-graph case.
                match g.order() > 0 {
                    false => Default::default(),
                    true => From::from([FromIterator::from_iter(g.vertices_iter())]),
                }
            },
        }
    }
}

impl<'a, T> Iterator for LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    type Item = (usize, &'a T::Vertex);

    fn next(&mut self) -> Option<Self::Item> {
        // While the queue is non-empty, select the first partition.
        if let Some(p) = self.partitions.front_mut() {
            // Select the first vertex from the partition.
            let x = p.pop_front().unwrap();
            // Add the selected vertex to the ordering.
            let s = Some((self.index, x));
            // Get neighbors of selected vertex.
            let mut neighbors: HashSet<_> = self.graph.neighbors_iter(x).collect();
            // The new partitioning ordering.
            let mut partitions: VecDeque<_> = Default::default();
            // Get iterator over the partitions.
            let mut iter = self.partitions.drain(..);
            // For each neighbor of the selected vertex ...
            while !neighbors.is_empty() {
                // ... for each partition in the queue.
                match iter.next() {
                    // No more partition to be checked, break,
                    None => break,
                    // Check next partition,
                    Some(p) => {
                        // For each vertex in the partition ...
                        let (p0, p1): (VecDeque<_>, VecDeque<_>) = p
                            .into_iter()
                            // ... split the partition into neighbor vs. non-neighbor
                            // vertices w.r.t. previously selected vertex.
                            .partition(|&y| neighbors.remove(y));
                        // If p0 is non-empty ...
                        if !p0.is_empty() {
                            // .. push it onto the queue.
                            partitions.push_back(p0);
                        }
                        // If p1 is non-empty ...
                        if !p1.is_empty() {
                            // ... push it onto the queue, after p0.
                            partitions.push_back(p1);
                        }
                    }
                }
            }
            // Append remaining partitions, if neighbor set was emptied
            // before reaching the end of the partitions queue.
            partitions.extend(iter);
            // Set new partitioning ordering.
            self.partitions = partitions;
            // Increase current index.
            self.index += 1;
            // Return lexicographic order.
            return s;
        }

        None
    }
}

impl<'a, T> FusedIterator for LexicographicBreadthFirstSearch<'a, T> where T: Undirected {}

impl<'a, T> From<&'a T> for LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    fn from(g: &'a T) -> Self {
        Self::new(g)
    }
}
