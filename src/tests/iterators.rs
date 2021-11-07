#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::{AdjacencyListGraph, GraphTrait};
    use crate::V;
    use all_asserts::*;

    const N: u32 = 1e3 as u32;

    #[test]
    fn vertex_iterator<T>()
    where
        T: GraphTrait<Vertex = u32, Edge = (u32, u32)>,
    {
        
    }

    #[instantiate_tests(<AdjacencyListGraph<u32>>)]
    mod adjacency_list_graph {}
}
