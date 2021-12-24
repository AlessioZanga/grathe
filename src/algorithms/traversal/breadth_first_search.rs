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
/// TODO: Once `min_specialization` and `Fn*` traits will be stabilized,
/// replace `from_*` using specialized `From` traits and
/// replace `run_*` using specialized `FnMut` traits.
///
#[derive(Default, Debug)]
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
    ($search:expr, $g:expr, $x:expr, $reachable:ident) => {{
        // Initialize the distance map.
        $search.distance.insert($x, 0);

        // Initialize the to-be-visited queue with the source vertex.
        let mut queue: VecDeque<_> = VecDeque::from([$x]);

        // Assert that source vertex is in graph.
        assert!($g.has_vertex($x));

        // While there are still vertices to be visited.
        while let Some(y) = queue.pop_front() {
            // Iterate over the reachable vertices of the popped vertex.
            for z in $reachable!($g, y).unwrap() {
                // If the vertex has never seen before.
                if !$search.distance.contains_key(z) {
                    // Compute the distance from its predecessor.
                    $search.distance.insert(z, $search.distance[y] + 1);
                    // Set its predecessor.
                    $search.predecessor.insert(z, y);
                    // Push it into the to-be-visited queue.
                    queue.push_back(z);
                }
            }
        }
    }};
}

impl<'a, T> BreadthFirstSearch<'a, T>
where
    T: VertexTrait,
{
    /// Run BFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_directed<U>(g: &'a U, x: &'a T) -> Self
    where
        U: DirectedTrait<Vertex = T>,
    {
        // Initialize the BFS object.
        let mut search: Self = Default::default();
        search.run_directed(g, x);
        search
    }

    /// Incrementally run BFS *tree* for a given directed graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn run_directed<U>(&mut self, g: &'a U, x: &'a T)
    where
        U: DirectedTrait<Vertex = T>,
    {
        bfs!(self, g, x, Ch);
    }

    /// Run BFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn from_undirected<U>(g: &'a U, x: &'a T) -> Self
    where
        U: UndirectedTrait<Vertex = T>,
    {
        // Initialize the BFS object.
        let mut search: Self = Default::default();
        search.run_undirected(g, x);
        search
    }

    /// Incrementally run BFS *tree* for a given undirected graph.
    ///
    /// # Panics
    ///
    /// Panics if the source vertex is not in the graph.
    ///
    pub fn run_undirected<U>(&mut self, g: &'a U, x: &'a T)
    where
        U: UndirectedTrait<Vertex = T>,
    {
        bfs!(self, g, x, Ne);
    }
}
