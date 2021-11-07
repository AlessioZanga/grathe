use crate::graphs::{AdjacencyListGraph, GraphTrait, VertexTrait};
use std::iter::Iterator;

#[derive(Debug)]
pub struct VertexIterator<'a, T>
where
    T: GraphTrait,
{
    data: &'a T::Storage,
}

impl<'a, T> VertexIterator<'a, AdjacencyListGraph<T>>
where
    T: VertexTrait,
{
    pub fn new(graph: &'a AdjacencyListGraph<T>) -> Self {
        VertexIterator { data: graph.data() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(|(v, _)| v)
    }
}

#[macro_export]
macro_rules! V {
    ( $x:expr ) => {
        {
            use crate::iterators::VertexIterator;
            VertexIterator::new($x)
        }
    };
}
