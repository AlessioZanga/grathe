use crate::directions::UndirectedTrait;
use crate::types::*;
use crate::Ne;
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

impl<'a, T> DepthFirstSearch<'a, T>
where
    T: VertexTrait,
{
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
        let mut discovery_time: HashMap<_, _> = HashMap::new();
        let mut finish_time: HashMap<_, _> = HashMap::new();
        let mut predecessor: HashMap<_, _> = HashMap::new();

        let mut time: usize = 0;
        let mut stack: Vec<_> = Vec::from([x]);

        assert!(g.has_vertex(x));

        // todo!():

        Self {
            discovery_time,
            finish_time,
            predecessor,
        }
    }
}
