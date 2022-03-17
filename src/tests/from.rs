#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::From;
    use crate::types::Error;

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn from_vertices<G>() -> Result<(), Error<i32>>
    where
        G: From<Vertex = i32>,
    {
        let mut g = G::from_vertices::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = G::from_vertices([0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = G::from_vertices([0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test next graph duplicated vertex set.
        let g = G::from_vertices([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);

        Ok(())
    }

    #[test]
    fn from_edges<G>() -> Result<(), Error<i32>>
    where
        G: From<Vertex = i32>,
    {
        let mut g = G::from_edges::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = G::from_edges([(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = G::from_edges(E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        // g = G::from_edges((0..N).zip(0..N));
        // assert_eq!(g.size(), &N);

        // Test next graph duplicated vertex set.
        let g = G::from_edges(E);
        assert_eq!(g.size(), 5);

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::From;
    use crate::types::Error;

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn from_vertices<G>() -> Result<(), Error<i32>>
    where
        G: From<Vertex = i32>,
    {
        let mut g = G::from_vertices::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.order(), 0);

        // Test next graph vertex set.
        g = G::from_vertices([0]);
        assert_eq!(g.order(), 1);

        // Test next graph unordered vertex set.
        g = G::from_vertices([0, 4, 2, 3, 1]);
        assert_eq!(g.order(), 5);

        // Test next graph duplicated vertex set.
        let g = G::from_vertices([0, 4, 2, 3, 1, 4, 3]);
        assert_eq!(g.order(), 5);

        Ok(())
    }

    #[test]
    fn from_edges<G>() -> Result<(), Error<i32>>
    where
        G: From<Vertex = i32>,
    {
        let mut g = G::from_edges::<_, i32>([]);

        // Test min graph vertex set.
        assert_eq!(g.size(), 0);

        // Test next graph vertex set.
        g = G::from_edges([(0, 0)]);
        assert_eq!(g.size(), 1);

        // Test next graph unordered vertex set.
        g = G::from_edges(E);
        assert_eq!(g.size(), 5);

        // Test high graph vertex set.
        // g = G::from_edges((0..N).zip(0..N));
        // assert_eq!(g.size(), &N);

        // Test next graph duplicated vertex set.
        let g = G::from_edges(E);
        assert_eq!(g.size(), 5);

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
