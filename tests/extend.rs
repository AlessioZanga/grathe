#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use grathe::graphs::storages::UndirectedAdjacencyList;
    use grathe::traits::Extend;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn extend_vertices<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_vertices(I: IntoIter<Item = i32>) -> bool`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence,
            (vec![], (vec![], false, 0)),
            // ... zero vertices and non-empty unique sequence,
            (vec![], (vec![0], true, 1)),
            // ... one vertex and non-empty unique sequence,
            (vec![0], (vec![1], true, 2)),
            // ... one vertex and long consecutive unique sequence,
            (vec![0], (Vec::from_iter(1..100), true, 100)),
            // ... one vertex and very-long consecutive unique sequence,
            (vec![0], (Vec::from_iter(1..10_000), true, 10_000)),
            // ... one vertex and very-long non-consecutive unique sequence,
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
            // ... one vertex and very-long non-consecutive repeated sequence,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter((1..10_000).chain(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    10_000,
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
    }

    #[test]
    fn extend_edges<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_edges(I: IntoIter<Item = (i32, i32)>) -> bool`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence,
            (vec![], (vec![], false, 0)),
            // ... one vertex and non-empty unique sequence,
            (vec![0], (vec![(0, 0)], true, 1)),
            // ... multiple vertices and long consecutive unique sequence,
            (Vec::from_iter(0..100), (Vec::from_iter((0..100).zip(1..100)), true, 99)),
            // ... multiple vertices and very-long consecutive unique sequence,
            (
                Vec::from_iter(0..10_000),
                (Vec::from_iter((0..10_000).zip(1..10_000)), true, 9_999),
            ),
            // ... multiple vertices and very-long non-consecutive unique sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    9_999,
                ),
            ),
            // ... multiple vertices and very-long non-consecutive repeated sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).zip(1..10_000)));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    9_999,
                ),
            ),
            // ... multiple vertices and very-long non-consecutive reversed sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).zip(1..10_000).map(|(x, y)| (y, x))));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    9_999,
                ),
            )
            // NOTE: This would panic!
            // ... one vertex and missing vertices,
            // (vec![0], (vec![(1, 1)], false, 0)),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, []);
            let (e, f, s) = j;
            assert_eq!(g.extend_edges(e), f);
            assert_eq!(g.size(), s);
        }
    }

    #[test]
    #[should_panic]
    fn extend_edges_should_panic<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_edges(I: IntoIter<Item = (i32, i32)>) -> bool`.

        let mut g = G::null();
        g.extend_edges([(0, 1)]);
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use grathe::graphs::storages::DirectedAdjacencyList;
    use grathe::traits::Extend;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn extend_vertices<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_vertices(I: IntoIter<Item = i32>) -> bool`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence,
            (vec![], (vec![], false, 0)),
            // ... zero vertices and non-empty unique sequence,
            (vec![], (vec![0], true, 1)),
            // ... one vertex and non-empty unique sequence,
            (vec![0], (vec![1], true, 2)),
            // ... one vertex and long consecutive unique sequence,
            (vec![0], (Vec::from_iter(1..100), true, 100)),
            // ... one vertex and very-long consecutive unique sequence,
            (vec![0], (Vec::from_iter(1..10_000), true, 10_000)),
            // ... one vertex and very-long non-consecutive unique sequence,
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
            // ... one vertex and very-long non-consecutive repeated sequence,
            (
                vec![0],
                (
                    {
                        let mut x = Vec::from_iter((1..10_000).chain(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    10_000,
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
    }

    #[test]
    fn extend_edges<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_edges(I: IntoIter<Item = (i32, i32)>) -> bool`.

        // Test for ...
        let data = [
            // ... zero vertices and empty sequence,
            (vec![], (vec![], false, 0)),
            // ... one vertex and non-empty unique sequence,
            (vec![0], (vec![(0, 0)], true, 1)),
            // ... multiple vertices and long consecutive unique sequence,
            (Vec::from_iter(0..100), (Vec::from_iter((0..100).zip(1..100)), true, 99)),
            // ... multiple vertices and very-long consecutive unique sequence,
            (
                Vec::from_iter(0..10_000),
                (Vec::from_iter((0..10_000).zip(1..10_000)), true, 9_999),
            ),
            // ... multiple vertices and very-long non-consecutive unique sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    9_999,
                ),
            ),
            // ... multiple vertices and very-long non-consecutive repeated sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).zip(1..10_000)));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    9_999,
                ),
            ),
            // ... multiple vertices and very-long non-consecutive reversed sequence,
            (
                Vec::from_iter(0..10_000),
                (
                    {
                        let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).rev().zip(1..10_000).rev()));
                        x.shuffle(&mut thread_rng());
                        x
                    },
                    true,
                    19_998,
                ),
            )
            // NOTE: This would panic!
            // ... one vertex and missing vertices,
            // (vec![0], (vec![(1, 1)], false, 0)),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, []);
            let (e, f, s) = j;
            assert_eq!(g.extend_edges(e), f);
            assert_eq!(g.size(), s);
        }
    }

    #[test]
    #[should_panic]
    fn extend_edges_should_panic<G>()
    where
        G: Extend<Vertex = i32>,
    {
        // Test `G::extend_edges(I: IntoIter<Item = (i32, i32)>) -> bool`.

        let mut g = G::null();
        g.extend_edges([(0, 1)]);
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
