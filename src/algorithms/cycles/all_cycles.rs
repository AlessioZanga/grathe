use crate::traits::Storage;
use crate::types::VertexIterator;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

/// Find all cycles.
///
/// Find all cycles in a graph[^note].
///
/// [^note]: [Hawick, K. A., & James, H. A. (2008). Enumerating Circuits and Loops in Graphs with Self-Arcs and Multiple-Arcs.](https://scholar.google.com/scholar?q=Enumerating+Circuits+and+Loops+in+Graphs+with+Self-Arcs+and+Multiple-Arcs)
///
pub struct AllCycles<'a, T>
where
    T: Storage,
{
    /// Given graph reference.
    graph: &'a T,
    /// Reachable vertices of distance one from given vertex.
    reachable: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    /// The currently visited stack.
    stack: Vec<&'a T::Vertex>,
    /// Map of *blocked* vertices in order to avoid double counting.
    blocked: HashMap<&'a T::Vertex, HashSet<&'a T::Vertex>>,
    /// Set of found cycles.
    pub cycles: Vec<Vec<&'a T::Vertex>>,
}

impl<'a, T> AllCycles<'a, T>
where
    T: Storage,
{
    pub fn new(g: &'a T, f: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>) -> Self {
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
            cycles: Default::default(),
        }
    }

    fn block(&mut self, v: &'a T::Vertex) {
        for w in (self.reachable)(self.graph, v) {
            if w < self.stack[0] {
                continue;
            }
            self.blocked.entry(w).or_default().insert(v);
        }
    }

    fn unblock(&mut self, u: &'a T::Vertex) {
        if let Some(u) = self.blocked.remove(u) {
            for w in u {
                self.unblock(w);
            }
        }
    }

    fn circuit(&mut self, v: &'a T::Vertex) -> bool {
        // Initialize found flag.
        let mut found = false;
        // Update the call stack.
        self.stack.push(v);
        // Initialize blocked map of current vertex.
        self.blocked.entry(v).or_default();

        // Iterate over reachable vertices from graph.
        for w in (self.reachable)(self.graph, v) {
            // If current vertex is lower then starting vertex,
            // then skip this iteration.
            if w < self.stack[0] {
                continue;
            // If current vertex is the starting vertex,
            // then a cycle has been found.
            } else if w == self.stack[0] {
                // Store the cycle.
                self.cycles.push({
                    // Clone the current stack.
                    let mut c = self.stack.clone();
                    // Add the leading vertex.
                    c.push(w);
                    // Return the completed cycle.
                    c
                });
                // Set the found flag.
                found = true;
            // Finally, if the current vertex has not been blocked...
            } else if !self.blocked.contains_key(&w) {
                // ... visit it recursively.
                found = self.circuit(w);
            }
        }

        match found {
            // If no cycle has been found, block the current vertex.
            false => self.block(v),
            // Otherwise, unblock it.
            true => self.unblock(v),
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
    pub fn run(&mut self) {
        for v in self.graph.vertices_iter() {
            // Visit current vertex recursively.
            self.circuit(v);
            // Clear map of blocked vertices.
            self.blocked.clear();
        }
    }
}
