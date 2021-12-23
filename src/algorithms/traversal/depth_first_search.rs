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
#[derive(Debug)]
pub struct DepthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Discovery time of each discovered vertex.
    pub discovery_time: HashMap<&'a T, usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: HashMap<&'a T, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T, &'a T>,
}

/// Reachable-agnostic implementation of DFS.
macro_rules! dfs {
    ($g:expr, $x:expr, $reachable:ident) => {{
        // Initialize the discovery-time map.
        let mut discovery_time: HashMap<_, _> = HashMap::new();
        // Initialize the finish-time map.
        let mut finish_time: HashMap<_, _> = HashMap::new();
        // Initialize the predecessors map.
        let mut predecessor: HashMap<_, _> = HashMap::new();

        // Initialize the global time counter.
        let mut time: usize = 0;
        // Initialize the visit stack.
        let mut stack: Vec<_> = Vec::from([$x]);

        // Assert that source vertex is in graph.
        assert!($g.has_vertex($x));

        // While there are still vertices to be visited.
        while let Some(&y) = stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if !discovery_time.contains_key(y) {
                // Set its discover time (as GRAY).
                discovery_time.insert(y, time);
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for z in $reachable!($g, y).unwrap() {
                    // Filter already visited vertices (as GRAY).
                    if !discovery_time.contains_key(z) {
                        // Set predecessor.
                        predecessor.insert(z, y);
                        // Add to queue.
                        queue.push_back(z);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                stack.extend(queue.iter().rev());
                // Increment time.
                time += 1;
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                stack.pop();
                // Check if it is GRAY (not BLACK).
                if !finish_time.contains_key(y) {
                    // Set its finish time (as BLACK).
                    finish_time.insert(y, time);
                    // Increment time.
                    time += 1;
                }
            }
        }

        Self {
            discovery_time,
            finish_time,
            predecessor,
        }
    }};
}

impl<'a, T> DepthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Execute DFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_directed<U>(g: &'a U, x: &'a T) -> Self
    where
        U: DirectedTrait<Vertex = T>,
    {
        dfs!(g, x, Ch)
    }

    /// Execute DFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_undirected<U>(g: &'a U, x: &'a T) -> Self
    where
        U: UndirectedTrait<Vertex = T>,
    {
        dfs!(g, x, Ne)
    }
}