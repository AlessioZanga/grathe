use crate::traits::Undirected;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::iter::FusedIterator;

/// Lexicographic depth-first search structure.
pub struct LexicographicDepthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Given graph reference.
    graph: &'a T,
    /// Current index.
    index: usize,
    /// To-be-visited queue.
    queue: HashMap<&'a T::Vertex, VecDeque<usize>>,
}

impl<'a, T> LexicographicDepthFirstSearch<'a, T>
where
    T: Undirected,
{
    /// Build a new LexDFS iterator.
    ///
    /// Build a LexDFS[^1] iterator for a given undirected graph.
    ///
    /// [^1]: [Corneil, D. G., & Krueger, R. M. (2008). A unified view of graph searching.](https://scholar.google.com/scholar?q=A+unified+view+of+graph+searching)
    ///
    pub fn new(g: &'a T) -> Self {
        Self {
            // Set target graph.
            graph: g,
            // Initialize index.
            index: Default::default(),
            // Initialize the to-be-visited queue with labels.
            queue: FromIterator::from_iter(g.vertices_iter().map(|x| (x, Default::default()))),
        }
    }
}

impl<'a, T> Iterator for LexicographicDepthFirstSearch<'a, T>
where
    T: Undirected,
{
    type Item = (usize, &'a T::Vertex);

    fn next(&mut self) -> Option<Self::Item> {
        // While the queue is non-empty.
        if !self.queue.is_empty() {
            // Select next vertex.
            let x = self
                .queue
                .iter()
                // Get min vertex with max label.
                .max_by(|x, y| match x.1.cmp(y.1) {
                    // If labels are equal, then prefer first found.
                    Ordering::Equal => y.0.cmp(x.0),
                    // Otherwise, return comparison result.
                    comparison => comparison,
                })
                .map(|(&x, _)| x)
                .unwrap();
            // Remove selected vertex from the visit queue.
            self.queue.remove(x);
            // Add the selected vertex to the ordering.
            let s = Some((self.index, x));
            // Iterate over vertex neighbors.
            for y in self.graph.neighbors_iter(x) {
                // If neighbor has not been visited yet.
                if let Some(y) = self.queue.get_mut(y) {
                    // Update neighbor label.
                    y.push_front(self.index);
                }
            }
            // Increase current index.
            self.index += 1;
            // Return lexicographic order.
            return s;
        }

        None
    }
}

impl<'a, T> FusedIterator for LexicographicDepthFirstSearch<'a, T> where T: Undirected {}

impl<'a, T> From<&'a T> for LexicographicDepthFirstSearch<'a, T>
where
    T: Undirected,
{
    fn from(g: &'a T) -> Self {
        Self::new(g)
    }
}
