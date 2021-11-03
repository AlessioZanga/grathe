#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::structs::AdjacencyListGraph;
    use crate::traits::Graph;

    #[test]
    fn eq<T: Graph>() {
        let g = T::new();
        let h = T::new();
        assert_eq!(g, h);
    }

    #[instantiate_tests(<AdjacencyListGraph>)]
    mod adjacency_list_graph {}
}
