use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    iter::FusedIterator,
};

use crate::{traits::Undirected, V, Ne};

/// Lexicographic depth-first search structure.
///
/// This structure contains the `predecessor` map.
///
pub struct LexicographicDepthFirstSearch<'a, G>
where
    G: Undirected,
{
    /// Given graph reference.
    graph: &'a G,
    /// Current index.
    index: usize,
    /// To-be-visited queue.
    queue: HashMap<&'a G::Vertex, VecDeque<usize>>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a G::Vertex, &'a G::Vertex>,
}

impl<'a, G> LexicographicDepthFirstSearch<'a, G>
where
    G: Undirected,
{
    /// Build a LexDFS iterator.
    ///
    /// Build a LexDFS[^1] iterator for a given undirected graph.
    ///
    /// This will execute the [`Forest`](super::Traversal) variant of the algorithm.
    ///
    /// [^1]: [Corneil, D. G., & Krueger, R. M. (2008). A unified view of graph searching.](https://scholar.google.com/scholar?q=A+unified+view+of+graph+searching)
    ///
    /// # Panics
    ///
    /// Panics if the (optional) source vertex is not in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    /// use grathe::algorithms::LexDFS;
    ///
    /// // Build an undirected graph.
    /// let g = Graph::from_edges([
    ///     (0, 1), (0, 2), (0, 4),
    ///     (1, 2), (1, 3),
    ///     (2, 4), (2, 3)
    /// ]);
    ///
    /// // Build a LexDFS iterator.
    /// let mut search = LexDFS::from(&g);
    ///
    /// // Check iterator order.
    /// assert_eq!(search.next(), Some(&0));
    /// assert_eq!(search.next(), Some(&1));
    /// assert_eq!(search.next(), Some(&2));
    /// assert_eq!(search.next(), Some(&3));
    /// assert_eq!(search.next(), Some(&4));
    /// assert_eq!(search.next(), None);
    ///
    /// // Check visiting tree.
    /// assert_eq!(search.predecessor.get(&0), None);
    /// assert_eq!(search.predecessor[&1], &0);
    /// assert_eq!(search.predecessor[&2], &1);
    /// assert_eq!(search.predecessor[&3], &2);
    /// assert_eq!(search.predecessor[&4], &2);
    /// ```
    ///
    pub fn new(g: &'a G, x: Option<&'a G::Vertex>) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            graph: g,
            // Initialize index.
            index: Default::default(),
            // Initialize the to-be-visited queue with labels.
            queue: HashMap::with_capacity(g.order()),
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
        // Add any vertex except the source vertex.
        search.queue.extend(V!(g).map(|x| (x, VecDeque::default())));
        // Push source vertex in front.
        search.queue.get_mut(x).unwrap().push_front(search.index);
        // Return search object.
        search
    }
}

impl<'a, G> Iterator for LexicographicDepthFirstSearch<'a, G>
where
    G: Undirected,
{
    type Item = &'a G::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        // While the queue is non-empty.
        if !self.queue.is_empty() {
            // Select next vertex.
            let x = self
                .queue
                .iter()
                // Get min vertex with max label.
                .max_by(|(x, x_label), (y, y_label)| match x_label.cmp(y_label) {
                    // If labels are equal, then prefer min vertex.
                    Ordering::Equal => y.cmp(x),
                    // Otherwise, return ordering result.
                    ordering => ordering,
                })
                .map(|(&x, _)| x)
                .unwrap();
            // Remove selected vertex from the visit queue.
            self.queue.remove(x);
            // Iterate over vertex neighbors.
            for y in Ne!(self.graph, x) {
                // If neighbor has not been visited yet.
                if let Some(y_label) = self.queue.get_mut(y) {
                    // Set its predecessor.
                    self.predecessor.insert(y, x);
                    // Update neighbor label.
                    y_label.push_front(self.index);
                }
            }
            // Increase current index.
            self.index += 1;
            // Return lexicographic order.
            return Some(x);
        }

        None
    }
}

impl<'a, G> FusedIterator for LexicographicDepthFirstSearch<'a, G> where G: Undirected {}

impl<'a, G> From<&'a G> for LexicographicDepthFirstSearch<'a, G>
where
    G: Undirected,
{
    /// Builds a search object from a given graph, without a source vertex.
    ///
    /// The first vertex of the vertex set is chosen as source vertex.
    ///
    fn from(g: &'a G) -> Self {
        Self::new(g, None)
    }
}

impl<'a, G> From<(&'a G, &'a G::Vertex)> for LexicographicDepthFirstSearch<'a, G>
where
    G: Undirected,
{
    /// Builds a search object from a given graph, with a source vertex.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    fn from((g, x): (&'a G, &'a G::Vertex)) -> Self {
        Self::new(g, Some(x))
    }
}
