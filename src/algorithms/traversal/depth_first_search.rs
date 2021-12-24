use crate::directions::{DirectedTrait, UndirectedTrait};
use crate::types::*;
use crate::{Ch, Ne};
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

/// Depth-first search structure.
///
/// This structure contains the `discovery_time`, `finish_time` and `predecessor` maps.
/// The underlying algorithm implements a depth-first search *tree* procedure,
/// where all-and-only the vertices reachable form a given source vertex are visited.
///
/// TODO: Once `min_specialization` and `Fn*` traits will be stabilized,
/// replace `from_*` using specialized `From` traits and
/// replace `run_*` using specialized `FnMut` traits.
///
#[derive(Default, Debug)]
pub struct DepthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Global time counter.
    pub time: usize,
    /// Discovery time of each discovered vertex.
    pub discovery_time: HashMap<&'a T, usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: HashMap<&'a T, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T, &'a T>,
}

/// Reachable-agnostic implementation of DFS.
macro_rules! dfs {
    ($search:expr, $g:expr, $x:expr, $reachable:ident) => {{
        // Initialize the visit stack.
        let mut stack: Vec<_> = Vec::from([$x]);

        // Assert that source vertex is in graph.
        assert!($g.has_vertex($x));

        // While there are still vertices to be visited.
        while let Some(&y) = stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if !$search.discovery_time.contains_key(y) {
                // Set its discover time (as GRAY).
                $search.discovery_time.insert(y, $search.time);
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for z in $reachable!($g, y).unwrap() {
                    // Filter already visited vertices (as GRAY).
                    if !$search.discovery_time.contains_key(z) {
                        // Set predecessor.
                        $search.predecessor.insert(z, y);
                        // Add to queue.
                        queue.push_back(z);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                stack.extend(queue.iter().rev());
                // Increment time.
                $search.time += 1;
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                stack.pop();
                // Check if it is GRAY (not BLACK).
                if !$search.finish_time.contains_key(y) {
                    // Set its finish time (as BLACK).
                    $search.finish_time.insert(y, $search.time);
                    // Increment time.
                    $search.time += 1;
                }
            }
        }
    }};
}

impl<'a, T> DepthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Run DFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_directed<U>(g: &'a U, x: &'a T) -> Self
    where
        U: DirectedTrait<Vertex = T>,
    {
        // Initialize the DFS object.
        let mut search: Self = Default::default();
        search.run_directed(g, x);
        search
    }

    /// Incrementally run DFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn run_directed<U>(&mut self, g: &'a U, x: &'a T)
    where
        U: DirectedTrait<Vertex = T>,
    {
        dfs!(self, g, x, Ch);
    }

    /// Run DFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_undirected<U>(g: &'a U, x: &'a T) -> Self
    where
        U: UndirectedTrait<Vertex = T>,
    {
        // Initialize the DFS object.
        let mut search: Self = Default::default();
        search.run_undirected(g, x);
        search
    }

    /// Incrementally run DFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn run_undirected<U>(&mut self, g: &'a U, x: &'a T)
    where
        U: UndirectedTrait<Vertex = T>,
    {
        dfs!(self, g, x, Ne);
    }
}
