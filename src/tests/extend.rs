#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Extend;
    use crate::types::Error;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn extend_vertices<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_vertices(I: IntoIter<Item = i32>) -> Result<()>`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence with no errors,
            (vec![], (vec![], true, 0)),
            // ... zero vertices and non-empty unique sequence with no errors,
            (vec![], (vec![0], true, 1)),
            // ... one vertex and non-empty unique sequence with no errors,
            (vec![0], (vec![1], true, 2)),
            // ... one vertex and long consecutive unique sequence with no errors,
            (vec![0], (Vec::from_iter(1..100), true, 100)),
            // ... one vertex and very-long consecutive unique sequence with no errors,
            (vec![0], (Vec::from_iter(1..10_000), true, 10_000)),
            // ... one vertex and very-long non-consecutive unique sequence with no errors,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter(1..10_000);
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    10_000,
                ),
            ),
            // ... one vertex and very-long non-consecutive repeated sequence with no errors,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter((1..10_000).chain(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    false,
                    1,
                ),
            ),
            // ... one vertex and a duplicated,
            (vec![0], (vec![0], false, 1)),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, []);
            let (v, f, o) = j;
            assert_eq!(g.extend_vertices(v), f);
            assert_eq!(g.order(), o);
        }

        Ok(())
    }

    #[test]
    // FIXME:
    fn extend_edges<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        todo!()
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
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn extend_vertices<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_vertices(I: IntoIter<Item = i32>) -> Result<()>`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence with no errors,
            (vec![], (vec![], true, 0)),
            // ... zero vertices and non-empty unique sequence with no errors,
            (vec![], (vec![0], true, 1)),
            // ... one vertex and non-empty unique sequence with no errors,
            (vec![0], (vec![1], true, 2)),
            // ... one vertex and long consecutive unique sequence with no errors,
            (vec![0], (Vec::from_iter(1..100), true, 100)),
            // ... one vertex and very-long consecutive unique sequence with no errors,
            (vec![0], (Vec::from_iter(1..10_000), true, 10_000)),
            // ... one vertex and very-long non-consecutive unique sequence with no errors,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter(1..10_000);
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    10_000,
                ),
            ),
            // ... one vertex and very-long non-consecutive repeated sequence with no errors,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter((1..10_000).chain(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    false,
                    1,
                ),
            ),
            // ... one vertex and a duplicated,
            (vec![0], (vec![0], false, 1)),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, []);
            let (v, f, o) = j;
            assert_eq!(g.extend_vertices(v), f);
            assert_eq!(g.order(), o);
        }

        Ok(())
    }

    #[test]
    // FIXME:
    fn extend_edges<G>() -> Result<(), Error<i32>>
    where
        G: Extend<Vertex = i32>,
    {
        todo!()
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
