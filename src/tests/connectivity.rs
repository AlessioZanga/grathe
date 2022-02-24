#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::errors::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::Connectivity;
    use all_asserts::*;

    #[test]
    fn has_path<T>() -> Result<(), Error<i32>>
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test self edge graph.
        let g = T::from_edges(&[(0, 0)]);
        assert_true!(g.has_path(&0, &0));

        // Test single edge graph.
        let g = T::from_edges(&[(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        // Test single path graph.
        let g = T::from_edges(&[(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_false!(g.has_path(&1, &0));
        assert_false!(g.has_path(&2, &0));
        assert_false!(g.has_path(&2, &1));

        // Test multi path graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));

        // Test disconnected graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        // Test cyclic graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4), (2, 0)]);
        assert_true!(g.has_path(&0, &0));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test null graph.
        let g = T::new();
        assert_false!(g.has_path(&0, &0));
    }

    #[test]
    fn is_connected<T>() -> Result<(), Error<i32>>
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test null graph.
        let g = T::new();
        assert_true!(g.is_connected());

        // Test single edge graph.
        let g = T::from_edges(&[(0, 1)]);
        assert_true!(g.is_connected());

        // Test multiple edges graph.
        let g = T::from_edges(&[(0, 1), (1, 2)]);
        assert_true!(g.is_connected());

        // Test disconnected graph.
        let mut g = T::from_edges(&[(0, 1), (1, 2)]);
        g.add_vertex(&3)?;
        assert_false!(g.is_connected());

        // Test connected cyclic graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        assert_true!(g.is_connected());

        // Test disconnected cyclic graph.
        let mut g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        g.add_vertex(&3)?;
        assert_false!(g.is_connected());

        Ok(())
    }

    #[test]
    fn is_acyclic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        let g = T::new();
        assert_true!(g.is_acyclic());

        let g = T::from_vertices(&[0]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1)]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1), (1, 2)]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        assert_false!(g.is_acyclic());
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32, (), (), ()>>)]
    mod adjacency_list_graph {}
}

#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::errors::Error;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::traits::Connectivity;
    use all_asserts::*;

    #[test]
    fn has_path<T>() -> Result<(), Error<i32>>
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test self edge graph.
        let g = T::from_edges(&[(0, 0)]);
        assert_true!(g.has_path(&0, &0));

        // Test single edge graph.
        let g = T::from_edges(&[(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        // Test single path graph.
        let g = T::from_edges(&[(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_true!(g.has_path(&1, &0));
        assert_true!(g.has_path(&2, &0));
        assert_true!(g.has_path(&2, &1));

        // Test multi path edge graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_true!(g.has_path(&2, &4));

        // Test disconnected graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        // Test cyclic graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4), (2, 0)]);
        println!("{:?}", g);
        assert_true!(g.has_path(&0, &0));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test null path.
        let g = T::new();
        assert_false!(g.has_path(&0, &0));
    }

    #[test]
    fn is_connected<T>() -> Result<(), Error<i32>>
    where
        T: Connectivity<Vertex = i32>,
    {
        // Test null graph.
        let g = T::new();
        assert_true!(g.is_connected());

        // Test single edge graph.
        let g = T::from_edges(&[(0, 1)]);
        assert_true!(g.is_connected());

        // Test multiple edges graph.
        let g = T::from_edges(&[(0, 1), (1, 2)]);
        assert_true!(g.is_connected());

        // Test disconnected graph.
        let mut g = T::from_edges(&[(0, 1), (1, 2)]);
        g.add_vertex(&3)?;
        assert_false!(g.is_connected());

        // Test connected cyclic graph.
        let g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        assert_true!(g.is_connected());

        // Test disconnected cyclic graph.
        let mut g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        g.add_vertex(&3)?;
        assert_false!(g.is_connected());

        Ok(())
    }

    #[test]
    #[ignore]
    fn is_acyclic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        let g = T::new();
        assert_true!(g.is_acyclic());

        let g = T::from_vertices(&[0]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1)]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1), (1, 2)]);
        assert_true!(g.is_acyclic());

        let g = T::from_edges(&[(0, 1), (1, 2), (2, 0)]);
        assert_false!(g.is_acyclic());
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32, (), (), ()>>)]
    mod adjacency_list_graph {}
}
