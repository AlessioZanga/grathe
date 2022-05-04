use std::{collections::HashSet, vec::Vec};

use crate::{
    traits::{PartiallyMixed, Storage},
    types::{directions, Mark as M},
    Adj,
};

/// Find all circle paths in a graph for given source and target vertices.
pub struct AllCirclePaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// To-be-visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a G::Vertex>,
    /// Vector of found circle paths.
    pub circle_paths: Vec<Vec<&'a G::Vertex>>,
}

impl<'a, G, D> AllCirclePaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Build an *all circle paths* search structure.
    ///
    /// Build an *all  circle paths* search structure which retains
    /// all the circle paths found for given source and target vertices.
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
            // Initialize vector of found circle paths.
            circle_paths: Default::default(),
        }
    }
}

impl<'a, G> AllCirclePaths<'a, G, directions::PartiallyMixed>
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
            // ... as X o-o Z.
            .filter(|z| self.g.has_mark(x, z, M::CircCirc))
        {
            // If the next vertex is the target.
            if z == y {
                // Then clones the stack into the results.
                self.circle_paths.push({
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

impl<'a, G, D> From<(&'a G, &'a G::Vertex, &'a G::Vertex)> for AllCirclePaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn from((g, x, y): (&'a G, &'a G::Vertex, &'a G::Vertex)) -> Self {
        Self::new(g, x, y)
    }
}
