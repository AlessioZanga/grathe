use std::{collections::HashSet, marker::PhantomData, vec::Vec};

use crate::{
    traits::{Directed, Storage, Undirected},
    types::directions,
    Ch, Ne,
};

/// Find all simple paths in a graph for given source and target vertices.
pub struct AllSimplePaths<'a, G, D>
where
    G: Storage,
{
    /// Given graph reference.
    graph: &'a G,
    /// To-be-visited stack.
    stack: Vec<&'a G::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a G::Vertex>,
    /// Vector of found simple paths.
    pub simple_paths: Vec<Vec<&'a G::Vertex>>,
    /// Generic placeholder for direction.
    _direction_type: PhantomData<D>,
}

impl<'a, G, D> AllSimplePaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    /// Build an *all simple paths* search structure.
    ///
    /// Build an *all  simple paths* search structure which retains
    /// all the simple paths found for given source and target vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use grathe::prelude::*;
    ///
    /// // Build a directed graph.
    /// let g = DiGraph::from_edges([
    ///     (0, 1), (0, 2), (0, 3),
    ///     (1, 2), (1, 3),
    ///     (2, 3)
    /// ]);
    ///
    /// // Build the search object over said graph.
    /// let mut search = AllSimplePaths::from((&g, &0, &3));
    ///
    /// // Run the algorithm and assert later.
    /// search.run();
    ///
    /// // In this graph there are four simple paths from `0` to `3`,
    /// // these are reported in discovery order.
    /// assert_eq!(
    ///     search.simple_paths,
    ///     [
    ///         vec![&0, &1, &2, &3],
    ///         vec![&0, &1, &3],
    ///         vec![&0, &2, &3],
    ///         vec![&0, &3],
    ///     ]
    /// );
    /// ```
    ///
    pub fn new(g: &'a G, x: &'a G::Vertex, y: &'a G::Vertex) -> Self {
        // Assert that source and target vertices are in graph.
        assert!(g.has_vertex(x) && g.has_vertex(y));

        Self {
            // Set target graph.
            graph: g,
            // Initialize the to-be-visited queue with source and target vertices.
            stack: From::from([x, y]),
            // Initialize the already visited set.
            visited: Default::default(),
            // Initialize vector of found simple paths.
            simple_paths: Default::default(),
            // Generic placeholder for direction.
            _direction_type: Default::default(),
        }
    }
}

impl<'a, G> AllSimplePaths<'a, G, directions::Undirected>
where
    G: Storage + Undirected,
{
    fn visit(&mut self, x: &'a G::Vertex, y: &'a G::Vertex) {
        // Push current vertex onto stack.
        self.stack.push(x);
        // Set current vertex as visited.
        self.visited.insert(x);
        // For each vertex reachable from the current vertex.
        for z in Ne!(self.graph, x) {
            // If the next vertex is the target.
            if z == y {
                // Then clones the stack into the results.
                self.simple_paths.push({
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
    pub fn run(&mut self) -> &Self {
        // Get target vertex.
        let y = self.stack.pop().unwrap();
        // Get source vertex.
        let x = self.stack.pop().unwrap();
        // Visit given graph.
        self.visit(x, y);

        self
    }
}

impl<'a, G> AllSimplePaths<'a, G, directions::Directed>
where
    G: Storage + Directed,
{
    fn visit(&mut self, x: &'a G::Vertex, y: &'a G::Vertex) {
        // Push current vertex onto stack.
        self.stack.push(x);
        // Set current vertex as visited.
        self.visited.insert(x);
        // For each vertex reachable from the current vertex.
        for z in Ch!(self.graph, x) {
            // If the next vertex is the target.
            if z == y {
                // Then clones the stack into the results.
                self.simple_paths.push({
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
    pub fn run(&mut self) -> &Self {
        // Get target vertex.
        let y = self.stack.pop().unwrap();
        // Get source vertex.
        let x = self.stack.pop().unwrap();
        // Visit given graph.
        self.visit(x, y);

        self
    }
}

impl<'a, G, D> From<(&'a G, &'a G::Vertex, &'a G::Vertex)> for AllSimplePaths<'a, G, D>
where
    G: Storage<Direction = D>,
{
    fn from((g, x, y): (&'a G, &'a G::Vertex, &'a G::Vertex)) -> Self {
        Self::new(g, x, y)
    }
}
