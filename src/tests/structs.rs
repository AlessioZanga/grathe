#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::structs::AdjacencyListGraph;
    use crate::traits::Graph;
    use all_asserts::*;

    #[test]
    fn eq<T: Graph>() {
        let mut g = T::new();
        let mut h = T::new();

        // Null graphs are equals by definition.
        assert_eq!(g, h); // G = (), H = ()

        // Two graphs are equals if the have the same vertex set and the same edge set.
        g.add_vertex(&0).unwrap();
        assert_ne!(g, h); // G = (0), H = ()

        h.add_vertex(&0).unwrap();
        assert_eq!(g, h); // G = (0), H = (0)

        g.add_edge(&(0, 0)).unwrap();
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0)

        h.add_vertex(&1).unwrap();
        assert_ne!(g, h); // G = (0, (0, 0)), H = (0, 1)
    }

    #[test]
    fn partial_cmp<T: Graph>() {
        let mut g = T::new();
        let h = T::new();

        // Null graphs are equals by definition.
        assert_false!(g < h);
        assert_true!(g <= h);
        assert_false!(g > h);
        assert_true!(g >= h);

        // The null graph is subgraph of every graph.
        g.add_vertex(&0).unwrap();
        assert_false!(g < h);
        assert_false!(g <= h);
        assert_true!(g > h);
        assert_true!(g >= h);
    }

    #[instantiate_tests(<AdjacencyListGraph>)]
    mod adjacency_list_graph {}
}
