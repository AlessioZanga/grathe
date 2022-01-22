use crate::traits::Storage;
use crate::types::VertexIterator;
use std::collections::HashSet;
use std::vec::Vec;

/// Find all simple paths in a graph for given source and target vertices.
pub struct AllSimplePaths<'a, T>
where
    T: Storage,
{
    /// Given graph reference.
    graph: &'a T,
    /// Reachable vertices of distance one from given vertex.
    reachable: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    /// To-be-visited stack.
    stack: Vec<&'a T::Vertex>,
    /// Already visited set.
    visited: HashSet<&'a T::Vertex>,
    /// Vector of found simple paths.
    pub paths: Vec<Vec<&'a T::Vertex>>,
}

impl<'a, T> AllSimplePaths<'a, T>
where
    T: Storage,
{
    pub fn new(
        g: &'a T,
        x: &'a T::Vertex,
        y: &'a T::Vertex,
        f: fn(&'a T, &'a T::Vertex) -> Box<dyn VertexIterator<'a, T::Vertex> + 'a>,
    ) -> Self {
        // Assert that source and target vertices are in graph.
        assert!(g.has_vertex(x) && g.has_vertex(y));

        Self {
            // Set target graph.
            graph: g,
            // Set reachability function.
            reachable: f,
            // Initialize the to-be-visited queue with the source vertex.
            stack: From::from([x, y]),
            // Initialize the already visited set.
            visited: Default::default(),
            // Initialize vector of found simple paths.
            paths: Default::default(),
        }
    }

    fn visit(&mut self, x: &'a T::Vertex, y: &'a T::Vertex) {
        self.stack.push(x);
        self.visited.insert(x);
        for z in (self.reachable)(self.graph, x) {
            if x == y {
                self.paths.push({
                    let mut p = self.stack.clone();
                    p.push(x);
                    p
                });
            } else if !self.visited.contains(x) {
                self.visit(z, y);
            }
        }
        self.visited.remove(x);
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
