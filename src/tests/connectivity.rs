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
        let g = T::from_edges(&[(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        let g = T::from_edges(&[(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_true!(g.has_path(&1, &0));
        assert_true!(g.has_path(&2, &0));
        assert_true!(g.has_path(&2, &1));

        let g = T::from_edges(&[(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_true!(g.has_path(&2, &4));

        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        let g = T::new();
        assert_false!(g.has_path(&0, &0));
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod undirected_adjacency_list_graph {}
}

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
        let g = T::from_edges(&[(0, 1)]);

        assert_false!(g.has_path(&0, &0));
        assert_true!(g.has_path(&0, &1));

        let g = T::from_edges(&[(0, 1), (1, 2)]);

        assert_true!(g.has_path(&0, &1));
        assert_true!(g.has_path(&1, &2));
        assert_false!(g.has_path(&1, &0));
        assert_false!(g.has_path(&2, &0));
        assert_false!(g.has_path(&2, &1));

        let g = T::from_edges(&[(0, 1), (1, 2), (1, 3), (3, 4)]);

        assert_true!(g.has_path(&0, &4));
        assert_true!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));

        let g = T::from_edges(&[(0, 1), (1, 2), (3, 4)]);

        assert_false!(g.has_path(&0, &4));
        assert_false!(g.has_path(&1, &4));
        assert_false!(g.has_path(&2, &4));
        assert_true!(g.has_path(&3, &4));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn has_path_should_panic<T>()
    where
        T: Connectivity<Vertex = i32>,
    {
        let g = T::new();
        assert_false!(g.has_path(&0, &0));
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod directed_adjacency_list_graph {}
}
