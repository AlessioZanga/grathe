use crate::traits::Undirected;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// Lexicographic breadth-first search structure.
pub struct LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Given graph reference.
    graph: &'a T,
    /// To-be-visited queue.
    queue: VecDeque<VecDeque<&'a T::Vertex>>,
    /// Current index.
    index: usize,
}

impl<'a, T> LexicographicBreadthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Build a new LexBFS iterator.
    ///
    /// Build a LexBFS[^1] iterator for a given graph.
    ///
    /// [^1]: [Bretscher, A., Corneil, D., Habib, M., & Paul, C. (2003, June). A simple linear time LexBFS cograph recognition algorithm.](https://scholar.google.com/scholar?q=+A+simple+linear+time+LexBFS+cograph+recognition+algorithm)
    ///
    pub fn new(g: &'a T) -> Self {
        Self {
            // Set target graph.
            graph: g,
            // Initialize the to-be-visited queue with the first partition.
            queue: {
                // Cover the null-graph case.
                match g.order() > 0 {
                    false => Default::default(),
                    true => From::from([FromIterator::from_iter(g.vertices_iter())]),
                }
            },
            // Initialize index.
            index: Default::default(),
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
        while let Some(p) = self.queue.front_mut() {
            // Select the first vertex from the partition.
            let x = p.pop_front().unwrap();
            // Add the selected vertex to the ordering.
            let s = Some((self.index, x));
            // Iterate over the queue.
            self.queue = self
                .queue
                .drain(..)
                // For each partition in the queue...
                .flat_map(|p| {
                    // ... for each vertex in the partition...
                    let (p0, p1): (VecDeque<_>, VecDeque<_>) = p
                        .into_iter()
                        // ... split the partition into adjacent vs. non-adjacent
                        // partitions w.r.t. previously selected vertex.
                        .partition(|&y| self.graph.has_edge(x, y).unwrap());
                    // Return splitted partitions.
                    [p0, p1]
                })
                // Ignore empty partitions.
                .filter(|p| !p.is_empty())
                // Push into queue.
                .collect();
            // Increase current index.
            self.index += 1;
            // Return ordering
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
