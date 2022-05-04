use std::{collections::HashSet, vec::Vec};

use crate::{
    traits::{PartiallyMixed, Storage},
    types::{directions, Mark as M},
    Adj,
};

/// Find all potentially directed paths in a graph for given source and target vertices.
pub struct AllPotentiallyDirectedPaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// To-be-visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a G::Vertex>,
    /// Vector of found potentially directed paths.
    pub potentially_directed_paths: Vec<Vec<&'a G::Vertex>>,
}

impl<'a, G, D> AllPotentiallyDirectedPaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Build an *all potentially directed paths* search structure.
    ///
    /// Build an *all  potentially directed paths* search structure which retains
    /// all the potentially directed paths found for given source and target vertices.
    ///
    pub fn new(g: &'a G, x: &'a G::Vertex, y: &'a G::Vertex) -> Self {
        // Assert that source and target vertices are in graph.
        assert!(g.has_vertex(x) && g.has_vertex(y));

        Self {
            // Set target graph.
            g,
            // Initialize the to-be-visited queue with source and target vertices.
            stack: From::from([x, y]),
            // Initialize the already visited set.
            visited: Default::default(),
            // Initialize vector of found potentially directed paths.
            potentially_directed_paths: Default::default(),
        }
    }
}

impl<'a, G> AllPotentiallyDirectedPaths<'a, G, directions::PartiallyMixed>
where
    G: Storage<Direction = directions::PartiallyMixed> + PartiallyMixed,
{
    fn visit(&mut self, x: &'a G::Vertex, y: &'a G::Vertex) {
        // Push current vertex onto stack.
        self.stack.push(x);
        // Set current vertex as visited.
        self.visited.insert(x);
        // For each vertex reachable from the current vertex ...
        for z in Adj!(self.g, x)
            // ... as ...
            .filter(|z| {
                // ... not *into* X, i.e. X <-* Z, and ...
                !matches!(self.g.get_mark(z, x), Some(M::CircHead | M::HeadHead | M::TailHead))
                // ... not *out of* Z, i.e. X *-- Z.
                && !matches!(self.g.get_mark(z, x), Some(M::TailHead | M::TailTail)) && !self.g.has_mark(x, z, M::CircTail)
            })
        {
            // If the next vertex is the target.
            if z == y {
                // Then clones the stack into the results.
                self.potentially_directed_paths.push({
                    let mut p = self.stack.clone();
                    p.push(z);
                    p
                });
            // Else if the next vertex has not been visited yet...
            } else if !self.visited.contains(z) {
                // ...call the visit procedure recursively.
                self.visit(z, y);
            }
        }
        // Set current vertex as not visited.
        self.visited.remove(x);
        // Pop current vertex from the stack.
        self.stack.pop();
    }

    /// Execute the procedure.
    ///
    /// Execute the procedure and store the results for later queries.
    ///
    /// TODO: Replace with FnMut once stabilized.
    ///
    pub fn call_mut(&mut self) -> &Self {
        // Get target vertex.
        let y = self.stack.pop().unwrap();
        // Get source vertex.
        let x = self.stack.pop().unwrap();
        // Visit given graph.
        self.visit(x, y);

        self
    }
}

impl<'a, G, D> From<(&'a G, &'a G::Vertex, &'a G::Vertex)> for AllPotentiallyDirectedPaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn from((g, x, y): (&'a G, &'a G::Vertex, &'a G::Vertex)) -> Self {
        Self::new(g, x, y)
    }
}
