use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    traits::{PartiallyDirected, PartiallyMixed, Storage},
    types::{directions, Mark as M},
    Adj, V,
};

/// Find all (minimal) discriminating paths in a graph.
pub struct AllDiscriminatingPaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// To-be-visited triplets.
    queue: Vec<(&'a G::Vertex, &'a G::Vertex, &'a G::Vertex)>,
    /// To-be-visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a G::Vertex>,
}

impl<'a, G, D> AllDiscriminatingPaths<'a, G, D>
where
    G: Storage<Direction = D> + PartiallyDirected,
{
    /// Build an *all (minimal) discriminating paths* search structure.
    pub fn new(g: &'a G) -> Self {
        // Find all valid triplets.
        let queue = V!(g)
            .permutations(3)
            .filter_map(|p| {
                // Get the triplet.
                let (x, y, z) = (p[0], p[1], p[2]);
                // Check if the triplet satisfy the initial discriminating path.
                if
                // X is a collider for Y, i.e. X <-* Y, and ...
                matches!(g.get_mark(y, x), Some(M::CircHead | M::HeadHead | M::TailHead))
                    // ... Y is adjacent to Z, and ...
                    && !(g.has_mark(y, z, M::None) || g.has_mark(z, y, M::None))
                    // ... X is a parent of Z.
                    && g.has_mark(x, z, M::TailHead)
                {
                    return Some((x, y, z));
                }

                None
            })
            .collect();

        Self {
            // Set target graph.
            g,
            // Initialize the to-be-visited queue triplets.
            queue,
            // Initialize the to-be-visited queue.
            stack: Default::default(),
            // Initialize the already visited set.
            visited: Default::default(),
        }
    }
}

impl<'a, G> AllDiscriminatingPaths<'a, G, directions::PartiallyMixed>
where
    G: Storage<Direction = directions::PartiallyMixed> + PartiallyMixed,
{
    /// Find a (minimal) discriminating path for X, Y and Z.
    ///
    /// The visit begins with an initial configuration of X, Y and Z such that:
    ///
    /// X <-> Y <-> Z and X --> Z
    ///
    /// which is the triangle structure that discriminates Y and sets Z as the
    /// endpoint of the discriminating path.
    ///
    /// Then, the path is extended backward, specifically every vertex W adjacent
    /// to X is checked to be a parent of X and non-adjacent to Y:
    ///
    /// W --> X and W -/- Y.
    ///
    /// If this is true, a discriminating path for X, Y and Z is found.
    /// Otherwise, the next adjacent vertex to X is checked.
    ///
    /// If no vertex W adjacent to X form a discriminating path, then:
    ///
    /// - if there is an unvisited vertex W adjacent to X that can extend
    ///   the current path, i.e. W <-> X and W --> Y, then repeat the visit
    ///   with the vertices adjacent to W,
    /// - otherwise, there is no discriminating path for X, Y and Z.
    ///
    fn visit(&mut self, x: &'a G::Vertex, y: &'a G::Vertex, z: &'a G::Vertex) -> Option<Vec<&'a G::Vertex>> {
        // Push current vertex onto stack.
        self.stack.push(x);
        // Set current vertex as visited.
        self.visited.insert(x);

        // For each vertex adjacent to the current vertex.
        for w in Adj!(self.g, x) {
            // If W has not been visited yet, and ...
            if !self.visited.contains(w) {
                // ... W -/- Z, and ...
                if (self.g.has_mark(w, z, M::None) && self.g.has_mark(z, w, M::None))
                    // ... W --> X ...
                    && self.g.has_mark(w, x, M::TailHead)
                // ... then W forms a discriminating path for X, Y and Z.
                {
                    // Add W to the path and ...
                    self.stack.push(w);
                    // Drain the stack in the reversed order.
                    return Some(self.stack.iter().rev().cloned().collect());
                // .. else if W extends the current path ...
                } else if
                // ... W --> Z, and ...
                self.g.has_mark(w, z, M::TailHead)
                    // ... W <-> X ...
                    && self.g.has_mark(w, x, M::HeadHead)
                {
                    // ... then, call the visit procedure recursively.
                    let discriminating_path = self.visit(w, y, z);
                    // If a path is found ...
                    if discriminating_path.is_some() {
                        // ... return recursively.
                        return discriminating_path;
                    }
                }
            }
        }

        // Set current vertex as not visited.
        self.visited.remove(x);
        // Pop current vertex from the stack.
        self.stack.pop();

        None
    }
}

impl<'a, G> Iterator for AllDiscriminatingPaths<'a, G, directions::PartiallyMixed>
where
    G: Storage<Direction = directions::PartiallyMixed> + PartiallyMixed,
{
    type Item = Vec<&'a G::Vertex>;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still triplets to be visited.
        while let Some((x, y, z)) = self.queue.pop() {
            // Set Y and Z as visited, X will be set immediately after the `visit` call.
            self.stack.extend([z, y]);
            self.visited.extend([y, z]);
            // Check if a discriminating path for (X, Y, Z) exists.
            let discriminating_path = self.visit(x, y, z);
            // Clear the stack.
            self.visited.clear();
            self.stack.clear();
            // If a path is found ...
            if discriminating_path.is_some() {
                // ... return recursively.
                return discriminating_path;
            }
        }

        None
    }
}
