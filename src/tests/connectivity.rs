#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::{Connectivity, From};
    use crate::types::Error;
    use all_asserts::*;

    #[test]
    fn has_path<G>() -> Result<(), Error<i32>>
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test self edge graph.
        let g = G::from_edges([(0, 0)]);
        assert_true!(g.has_path(&0, &0));

        // Test single edge graph.
        let g = G::from_edges([(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        // Test single path graph.
        let g = G::from_edges([(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_true!(g.has_path(&1, &0));
        assert_true!(g.has_path(&2, &0));
        assert_true!(g.has_path(&2, &1));

        // Test multi path edge graph.
        let g = G::from_edges([(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_true!(g.has_path(&2, &4));

        // Test disconnected graph.
        let g = G::from_edges([(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        // Test cyclic graph.
        let g = G::from_edges([(0, 1), (1, 2), (3, 4), (2, 0)]);
        println!("{:?}", g);
        assert_true!(g.has_path(&0, &0));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<G>()
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test null path.
        let g = G::null();
        assert_false!(g.has_path(&0, &0));
    }

    #[test]
    fn is_connected<G>() -> Result<(), Error<i32>>
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test null graph.
        let g = G::null();
        assert_true!(g.is_connected());

        // Test single edge graph.
        let g = G::from_edges([(0, 1)]);
        assert_true!(g.is_connected());

        // Test multiple edges graph.
        let g = G::from_edges([(0, 1), (1, 2)]);
        assert_true!(g.is_connected());

        // Test disconnected graph.
        let mut g = G::from_edges([(0, 1), (1, 2)]);
        g.add_vertex(3)?;
        assert_false!(g.is_connected());

        // Test connected cyclic graph.
        let g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        assert_true!(g.is_connected());

        // Test disconnected cyclic graph.
        let mut g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        g.add_vertex(3)?;
        assert_false!(g.is_connected());

        Ok(())
    }

    #[test]
    #[ignore]
    fn is_acyclic<G>()
    where
        G: Connectivity<Vertex = i32> + From,
    {
        let g = G::null();
        assert_true!(g.is_acyclic());

        let g = G::from_vertices([0]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1)]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1), (1, 2)]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        assert_false!(g.is_acyclic());
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::{Connectivity, From};
    use crate::types::Error;
    use all_asserts::*;

    #[test]
    fn has_path<G>() -> Result<(), Error<i32>>
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test self edge graph.
        let g = G::from_edges([(0, 0)]);
        assert_true!(g.has_path(&0, &0));

        // Test single edge graph.
        let g = G::from_edges([(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        // Test single path graph.
        let g = G::from_edges([(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_false!(g.has_path(&1, &0));
        assert_false!(g.has_path(&2, &0));
        assert_false!(g.has_path(&2, &1));

        // Test multi path graph.
        let g = G::from_edges([(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));

        // Test disconnected graph.
        let g = G::from_edges([(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        // Test cyclic graph.
        let g = G::from_edges([(0, 1), (1, 2), (3, 4), (2, 0)]);
        assert_true!(g.has_path(&0, &0));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<G>()
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test null graph.
        let g = G::null();
        assert_false!(g.has_path(&0, &0));
    }

    #[test]
    fn is_connected<G>() -> Result<(), Error<i32>>
    where
        G: Connectivity<Vertex = i32> + From,
    {
        // Test null graph.
        let g = G::null();
        assert_true!(g.is_connected());

        // Test single edge graph.
        let g = G::from_edges([(0, 1)]);
        assert_true!(g.is_connected());

        // Test multiple edges graph.
        let g = G::from_edges([(0, 1), (1, 2)]);
        assert_true!(g.is_connected());

        // Test disconnected graph.
        let mut g = G::from_edges([(0, 1), (1, 2)]);
        g.add_vertex(3)?;
        assert_false!(g.is_connected());

        // Test connected cyclic graph.
        let g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        assert_true!(g.is_connected());

        // Test disconnected cyclic graph.
        let mut g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        g.add_vertex(3)?;
        assert_false!(g.is_connected());

        Ok(())
    }

    #[test]
    fn is_acyclic<G>()
    where
        G: Connectivity<Vertex = i32> + From,
    {
        let g = G::null();
        assert_true!(g.is_acyclic());

        let g = G::from_vertices([0]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1)]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1), (1, 2)]);
        assert_true!(g.is_acyclic());

        let g = G::from_edges([(0, 1), (1, 2), (2, 0)]);
        assert_false!(g.is_acyclic());
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
