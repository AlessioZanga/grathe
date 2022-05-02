use std::collections::HashSet;

use crate::{
    traits::{PartiallyMixed, Storage},
    types::{directions, Marker},
    Adj,
};

/// Find all (minimal) discriminating paths in a graph.
pub struct AllDiscriminatingPaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// To-be-visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a G::Vertex>,
}

impl<'a, G> AllDiscriminatingPaths<'a, G, directions::PartiallyMixed>
where
    G: Storage<Direction = directions::PartiallyMixed> + PartiallyMixed,
{
    /// Find a discriminating path for X, Y and Z.
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
    fn visit(&mut self, x: &'a G::Vertex, z: &'a G::Vertex) -> Option<Vec<&'a G::Vertex>> {
        // Push current vertex onto stack.
        self.stack.push(x);
        // Set current vertex as visited.
        self.visited.insert(x);
        // For each vertex adjacent to the current vertex.
        for w in Adj!(self.g, x) {
            // If W has not been visited yet, and ...
            if !self.visited.contains(w) {
                // ... W -/- Z, and ...
                if self.g.has_marker(w, z, Marker::None)
                    && self.g.has_marker(z, w, Marker::None)
                    && matches!(
                        // ... W *-> X ...
                        self.g.get_marker(w, x).unwrap(),
                        Marker::CircHead | Marker::HeadHead | Marker::TailHead
                    )
                // ... then W forms a discriminating path for X, Y and Z.
                {
                    // Add W to the path and ...
                    self.stack.push(w);
                    // Clear the visited set.
                    self.visited.clear();
                    // Drain the stack in the reversed order.
                    return Some(self.stack.drain(..).rev().collect());
                // .. else if W extends the current path ...
                } else if
                    // ... W --> Z, and ...
                    self.g.has_marker(w, z, Marker::TailHead) 
                    // ... W <-> X ...
                    && self.g.has_marker(w, x, Marker::HeadHead) {
                    // ... then, call the visit procedure recursively.
                    let discriminating_path = self.visit(w, z);
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
