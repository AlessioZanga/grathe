use crate::delegate_storage;
use crate::graphs::UndirectedTrait;
use crate::storages::{AdjacencyListStorage, StorageTrait};
use crate::types::*;

#[derive(Debug)]
pub struct UndirectedAdjacencyListGraph<T>(AdjacencyListStorage<T>)
where
    T: VertexTrait;

delegate_storage!(UndirectedAdjacencyListGraph);

// impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T> where T: VertexTrait {}
