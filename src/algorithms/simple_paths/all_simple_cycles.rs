use crate::traits::Storage;
use crate::types::VertexIterator;
use crate::V;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

/// Find all cycles in a given graph.
///
/// Find all cycles in a given graph (or a multi-graph[^1]). This algorithm [^2]
/// returns all elementary circuits (i.e. *cycles*) present in a graph.
/// The graph can contain self-edges and can also be disconnected.
///
/// [^1]: TODO: Currently, this specific implementation does not work in multi-graphs.
/// However, once multi-graphs will be supported library-wide, it will be restored to its original formulation.
///
/// [^2]: [Hawick, K. A., & James, H. A. (2008). Enumerating Circuits and Loops in Graphs with Self-Arcs and Multiple-Arcs.](https://scholar.google.com/scholar?q=Enumerating+Circuits+and+Loops+in+Graphs+with+Self-Arcs+and+Multiple-Arcs)
///
pub struct AllSimpleCycles<'a, G>
where
    G: Storage,
{
    /// Given graph reference.
    graph: &'a G,
    /// Reachable vertices of distance one from given vertex.
    reachable: fn(&'a G, &'a G::Vertex) -> Box<dyn VertexIterator<'a, G::Vertex> + 'a>,
    /// The currently visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Map of *blocked* vertices in order to avoid double counting.
    // TODO: Replace HashSet with a Vec to account for multi-graphs once supported.
    blocked: HashMap<&'a G::Vertex, HashSet<&'a G::Vertex>>,
    /// Vector of found simple cycles.
    pub simple_cycles: Vec<Vec<&'a G::Vertex>>,
    /// Map of vertices popularity (i.e. how many cycles a vertex appears in).
    pub popularity: HashMap<&'a G::Vertex, usize>,
}

impl<'a, G> AllSimpleCycles<'a, G>
where
    G: Storage,
{
    /// Build an *all cycles* search structure.
    ///
    /// Build an *all cycles* search structure which retains the cycles found,
    /// together with the popularity count of each vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a directed graph.
    /// let g = DiGraph::from_edges([
    ///     (0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (4, 1)
    /// ]);
    ///
    /// // Build the search object over said graph.
    /// let mut search = AllSimpleCycles::from(&g);
    ///
    /// // Run the algorithm and assert later.
    /// search.run();
    ///
    /// // In this graph there are two directed cycles,
    /// // these are reported in discovery order.
    /// assert_eq!(
    ///     search.simple_cycles,
    ///     [
    ///         vec![&1, &2, &3, &4, &1],
    ///         vec![&1, &2, &4, &1]
    ///     ]
    /// );
    ///
    /// // Vertex `0` is not present in any cycle,
    /// // therefore it is not in the popularity map.
    /// assert_eq!(search.popularity.get(&0), None);
    /// // Vertices `1`, `2` and `4` are present in both cycles,
    /// // while vertex `3` is present only in the first one.
    /// assert_eq!(search.popularity[&1], 2);
    /// assert_eq!(search.popularity[&2], 2);
    /// assert_eq!(search.popularity[&3], 1);
    /// assert_eq!(search.popularity[&4], 2);
    /// ```
    ///
    pub fn new(g: &'a G, f: fn(&'a G, &'a G::Vertex) -> Box<dyn VertexIterator<'a, G::Vertex> + 'a>) -> Self {
        Self {
            // Set target graph.
            graph: g,
            // Set reachability function.
            reachable: f,
            // Initialize the currently visited stack.
            stack: Default::default(),
            // Initialize blocked map.
            blocked: Default::default(),
            // Initialize cycles set.
            simple_cycles: Default::default(),
            // Initialize popularity map.
            popularity: Default::default(),
        }
    }

    fn block(&mut self, x: &'a G::Vertex) {
        for y in (self.reachable)(self.graph, x) {
            if y < self.stack[0] {
                continue;
            }
            self.blocked.entry(y).or_default().insert(x);
        }
    }

    fn unblock(&mut self, x: &'a G::Vertex) {
        if let Some(y) = self.blocked.remove(x) {
            y.iter().for_each(|y| self.unblock(y));
        }
    }

    fn circuit(&mut self, x: &'a G::Vertex) -> bool {
        // Initialize found flag.
        let mut found = false;
        // Update the call stack.
        self.stack.push(x);
        // Initialize blocked map of current vertex.
        self.blocked.entry(x).or_default();

        // Iterate over reachable vertices from graph.
        for y in (self.reachable)(self.graph, x) {
            // If current vertex is lower then starting vertex,
            // then skip this iteration.
            if y < self.stack[0] {
                continue;
            // If current vertex is the starting vertex,
            // then a cycle has been found.
            } else if y == self.stack[0] {
                // Store the cycle.
                self.simple_cycles.push({
                    // Clone the current stack.
                    let mut c = self.stack.clone();
                    // Add the leading vertex.
                    c.push(y);
                    // Return the completed cycle.
                    c
                });
                // Update popularity.
                for z in self.stack.iter() {
                    *self.popularity.entry(z).or_default() += 1;
                }
                // Set the found flag.
                found = true;
            // Finally, if the current vertex has not been blocked...
            } else if !self.blocked.contains_key(&y) {
                // ...visit it recursively.
                found |= self.circuit(y);
            }
        }

        match found {
            // If no cycle has been found, block the current vertex.
            false => self.block(x),
            // Otherwise, unblock it.
            true => self.unblock(x),
        };

        // Pop from stack.
        self.stack.pop();

        // Return found flag.
        found
    }

    /// Execute the procedure.
    ///
    /// Execute the procedure and store the results for later queries.
    ///
    pub fn run(&mut self) -> &Self {
        for x in V!(self.graph) {
            // Visit current vertex recursively.
            self.circuit(x);
            // Clear map of blocked vertices.
            self.blocked.clear();
        }

        self
    }
}
