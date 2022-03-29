#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Subgraph;
    use crate::types::Error;
    use crate::{E, V};
    use all_asserts::*;

    #[test]
    #[ignore]
    // FIXME:
    fn subgraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1), (0, 2), (1, 2), (2, 3), (3, 3)]);

        // Build subgraph over 0, 2 and 3.
        let h = g.subgraph([0, 2, 3]);

        // Check if it is a subgraph.
        assert_le!(h, g);

        // Check if only selected vertices are preserved.
        assert_true!(V!(h).eq(&[0, 2, 3]));

        // Check if only associated edges are preserved.
        assert_true!(E!(h).eq([(&0, &2), (&2, &3), (&3, &3)]));

        Ok(())
    }

    #[test]
    #[ignore]
    // FIXME:
    fn is_subgraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1)]);
        let h = G::new([], [(0, 1), (0, 2)]);

        assert_le!(g, h);
        assert_eq!((g <= h), g.is_subgraph(&h));

        Ok(())
    }

    #[test]
    #[ignore]
    // FIXME:
    fn is_supergraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1), (0, 2)]);
        let h = G::new([], [(0, 1)]);

        assert_ge!(g, h);
        assert_eq!((g >= h), g.is_supergraph(&h));

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::Subgraph;
    use crate::types::Error;
    use crate::{E, V};
    use all_asserts::*;

    #[test]
    #[ignore]
    // FIXME:
    fn subgraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1), (0, 2), (1, 2), (2, 3), (3, 3)]);

        // Build subgraph over 0, 2 and 3.
        let h = g.subgraph([0, 2, 3]);

        // Check if it is a subgraph.
        assert_le!(h, g);

        // Check if only selected vertices are preserved.
        assert_true!(V!(h).eq(&[0, 2, 3]));

        // Check if only associated edges are preserved.
        assert_true!(E!(h).eq([(&0, &2), (&2, &3), (&3, &3)]));

        Ok(())
    }

    #[test]
    #[ignore]
    // FIXME:
    fn is_subgraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1)]);
        let h = G::new([], [(0, 1), (0, 2)]);

        assert_le!(g, h);
        assert_eq!((g <= h), g.is_subgraph(&h));

        Ok(())
    }

    #[test]
    #[ignore]
    // FIXME:
    fn is_supergraph<G>() -> Result<(), Error<i32>>
    where
        G: Subgraph<Vertex = i32>,
    {
        let g = G::new([], [(0, 1), (0, 2)]);
        let h = G::new([], [(0, 1)]);

        assert_ge!(g, h);
        assert_eq!((g >= h), g.is_supergraph(&h));

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
