#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Extend;
    use crate::types::Error;
    use all_asserts::*;

    #[test]
    // FIXME:
    fn extend_vertices<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        let mut g = G::null();

        // Extend graph with vertices.
        g.extend_vertices([0, 3, 1, 2])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 0);
        // Extending with existing vertices yields an error.
        assert_true!(g.extend_vertices([0]).is_err());
        Ok(())
    }

    #[test]
    // FIXME:
    fn extend_edges<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        let mut g = G::null();

        // Extend graph with edges.
        g.extend_edges([(0, 3), (1, 2)])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 2);

        // Extending with existing edges yields an error.
        assert_true!(g.extend_edges([(0, 3)]).is_err());
        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::Extend;
    use crate::types::Error;
    use all_asserts::*;

    #[test]
    // FIXME:
    fn extend_vertices<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        let mut g = G::null();

        // Extend graph with vertices.
        g.extend_vertices([0, 3, 1, 2])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 0);
        // Extending with existing vertices yields an error.
        assert_true!(g.extend_vertices([0]).is_err());
        Ok(())
    }

    #[test]
    // FIXME:
    fn extend_edges<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        let mut g = G::null();

        // Extend graph with edges.
        g.extend_edges([(0, 3), (1, 2)])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 2);

        // Extending with existing edges yields an error.
        assert_true!(g.extend_edges([(0, 3)]).is_err());
        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
