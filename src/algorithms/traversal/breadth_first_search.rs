use crate::directions::{DirectedTrait, UndirectedTrait};
use crate::types::*;
use crate::{Ch, Ne};
use std::collections::{HashMap, VecDeque};

/// Breadth-first search structure.
///
/// This structure contains the `distance` and `predecessor` maps.
/// The underlying algorithm implements a breadth-first search *tree* procedure,
/// where all-and-only the vertices reachable form a given source vertex are visited.
///
#[derive(Debug)]
pub struct BreadthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Distance from the source vertex.
    pub distance: HashMap<&'a T, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<&'a T, &'a T>,
}

/// Reachable-agnostic implementation of BFS.
macro_rules! bfs {
    ($g:expr, $x:expr, $reachable:ident) => {{
        // Initialize the distance map.
        let mut distance: HashMap<_, _> = HashMap::from([($x, 0)]);
        // Initialize the predecessor map.
        let mut predecessor: HashMap<_, _> = HashMap::new();

        // Initialize the to-be-visited queue with the source vertex.
        let mut queue: VecDeque<_> = VecDeque::from([$x]);

        // Assert that source vertex is in graph.
        assert!($g.has_vertex($x));

        // While there are still vertices to be visited.
        while let Some(y) = queue.pop_front() {
            // Iterate over the reachable vertices of the popped vertex.
            for z in $reachable!($g, y).unwrap() {
                // If the vertex has never seen before.
                if !distance.contains_key(z) {
                    // Compute the distance from its predecessor.
                    distance.insert(z, distance[y] + 1);
                    // Set its predecessor.
                    predecessor.insert(z, y);
                    // Push it into the to-be-visited queue.
                    queue.push_back(z);
                }
            }
        }

        Self { distance, predecessor }
    }};
}

impl<'a, T> BreadthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Execute BFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_directed<U>(g: &'a U, x: &'a T) -> Self
    where
        U: DirectedTrait<Vertex = T>,
    {
        bfs!(g, x, Ch)
    }

    /// Execute BFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_undirected<U>(g: &'a U, x: &'a T) -> Self
    where
        U: UndirectedTrait<Vertex = T>,
    {
        bfs!(g, x, Ne)
    }
}
