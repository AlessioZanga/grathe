use crate::traits::Undirected;
use crate::V;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FusedIterator;

/// Lexicographic breadth-first search structure.
///
/// This structure contains the `predecessor` map.
///
pub struct LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Given graph reference.
    graph: &'a T,
    /// To-be-visited queue.
    pub partitions: VecDeque<VecDeque<&'a T::Vertex>>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T::Vertex, &'a T::Vertex>,
}

impl<'a, T> LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Build a LexBFS iterator.
    ///
    /// Build a LexBFS[^1] iterator for a given undirected graph.
    ///
    /// This will execute the [`Forest`](super::Traversal) variant of the algorithm.
    ///
    /// [^1]: [Bretscher, A., Corneil, D., Habib, M., & Paul, C. (2003, June). A simple linear time LexBFS cograph recognition algorithm.](https://scholar.google.com/scholar?q=+A+simple+linear+time+LexBFS+cograph+recognition+algorithm)
    ///
    /// # Panics
    ///
    /// Panics if the (optional) source vertex is not in the graph.
    ///
    pub fn new(g: &'a T, x: Option<&'a T::Vertex>) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Initialize the to-be-visited queue with the first partition, if any.
            partitions: Default::default(),
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
        // Initialize partition with predefined capacity.
        let mut partition = VecDeque::with_capacity(g.order());
        // Add any vertex except the source vertex.
        partition.extend(V!(g).filter(|&y| y != x));
        // Push source vertex in front.
        partition.push_front(x);
        // Set partition as first partition.
        search.partitions.push_front(partition);
        // Return search object.
        search
    }
}

impl<'a, T> Iterator for LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    type Item = &'a T::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // While the queue is non-empty, select the first partition.
        if let Some(p) = self.partitions.front_mut() {
            // Select the first vertex from the partition.
            let x = p.pop_front().unwrap();
            // Get neighbors of selected vertex.
            let mut neighbors: HashSet<_> = self.graph.neighbors_iter(x).collect();
            // Initialize the new partitioning ordering.
            // TODO: Replace the partitions queue with a vector and begin/end
            // indices pairs once `partition_in_place` is stabilized.
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
                            .partition(|&y| {
                                // If `y` is a neighbor of `x` and it is still in a partition,
                                // i.e. it was not visited before ...
                                if neighbors.remove(y) {
                                    // ... set its predecessor, if not already present.
                                    self.predecessor.entry(y).or_insert(x);
                                    // ... and return true.
                                    return true;
                                }
                                // Otherwise, return false.
                                false
                            });
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
            // Return lexicographic order.
            return Some(x);
        }

        None
    }
}

impl<'a, T> FusedIterator for LexicographicBreadthFirstSearch<'a, T> where T: Undirected {}

impl<'a, T> From<&'a T> for LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Builds a search object from a given graph, without a source vertex.
    ///
    /// The first vertex of the vertex set is chosen as source vertex.
    ///
    fn from(g: &'a T) -> Self {
        Self::new(g, None)
    }
}

impl<'a, T> From<(&'a T, &'a T::Vertex)> for LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Builds a search object from a given graph, with a source vertex.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    fn from((g, x): (&'a T, &'a T::Vertex)) -> Self {
        Self::new(g, Some(x))
    }
}
