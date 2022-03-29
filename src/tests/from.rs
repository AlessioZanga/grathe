#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::From;
    use crate::{E, V};
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn from_vertices<G>()
    where
        G: From<Vertex = i32>,
    {
        // Test `G::from_vertices(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... empty sequence,
            (vec![], (vec![], 0)),
            // ... non-empty unique sequence,
            (vec![0], (vec![0], 1)),
            // ... long consecutive unique sequence,
            (Vec::from_iter(0..100), (Vec::from_iter(0..100), 100)),
            // ... very-long consecutive unique sequence,
            (Vec::from_iter(0..10_000), (Vec::from_iter(0..10_000), 10_000)),
            // ... very-long non-consecutive unique sequence,
            (
                {
                    let mut x = Vec::from_iter(0..10_000);
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter(0..10_000), 10_000),
            ),
            // ... very-long non-consecutive repeated sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).chain(1..10_000));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter(0..10_000), 10_000),
            ),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let g = G::from_vertices(i);
            let (v, o) = j;
            assert!(V!(g).eq(&v));
            assert_eq!(g.order(), o);
        }
    }

    #[test]
    fn from_edges<G>()
    where
        G: From<Vertex = i32>,
    {
        // Test `G::from_edges(I: IntoIter<Item = (i32, i32)>) -> Self`.

        // Test for ...
        let data = [
            // ... empty sequence,
            (vec![], (vec![], 0)),
            // ... non-empty unique sequence,
            (vec![(0, 0)], (vec![(0, 0)], 1)),
            // ... long consecutive unique sequence,
            (
                Vec::from_iter((0..100).zip(1..100)),
                (Vec::from_iter((0..100).zip(1..100)), 99),
            ),
            // ... very-long consecutive unique sequence,
            (
                Vec::from_iter((0..10_000).zip(1..10_000)),
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive unique sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).zip(1..10_000));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive repeated sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).zip(1..10_000)));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive reversed sequence,
            (
                {
                    let mut x = Vec::from_iter(
                        (0..10_000)
                            .zip(1..10_000)
                            .chain((0..10_000).zip(1..10_000).map(|(x, y)| (y, x))),
                    );
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
        ];

        // Test for each scenario.
        for (j, k) in data {
            let g = G::from_edges(j);
            let (e, s) = k;
            assert!(E!(g).eq(e.iter().map(|e| (&e.0, &e.1))));
            assert_eq!(g.size(), s);
        }
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::From;
    use crate::{E, V};
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn from_vertices<G>()
    where
        G: From<Vertex = i32>,
    {
        // Test `G::from_vertices(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... empty sequence,
            (vec![], (vec![], 0)),
            // ... non-empty unique sequence,
            (vec![0], (vec![0], 1)),
            // ... long consecutive unique sequence,
            (Vec::from_iter(0..100), (Vec::from_iter(0..100), 100)),
            // ... very-long consecutive unique sequence,
            (Vec::from_iter(0..10_000), (Vec::from_iter(0..10_000), 10_000)),
            // ... very-long non-consecutive unique sequence,
            (
                {
                    let mut x = Vec::from_iter(0..10_000);
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter(0..10_000), 10_000),
            ),
            // ... very-long non-consecutive repeated sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).chain(1..10_000));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter(0..10_000), 10_000),
            ),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let g = G::from_vertices(i);
            let (v, o) = j;
            assert!(V!(g).eq(&v));
            assert_eq!(g.order(), o);
        }
    }

    #[test]
    fn from_edges<G>()
    where
        G: From<Vertex = i32>,
    {
        // Test `G::from_edges(I: IntoIter<Item = (i32, i32)>) -> Self`.

        // Test for ...
        let data = [
            // ... empty sequence,
            (vec![], (vec![], 0)),
            // ... non-empty unique sequence,
            (vec![(0, 0)], (vec![(0, 0)], 1)),
            // ... long consecutive unique sequence,
            (
                Vec::from_iter((0..100).zip(1..100)),
                (Vec::from_iter((0..100).zip(1..100)), 99),
            ),
            // ... very-long consecutive unique sequence,
            (
                Vec::from_iter((0..10_000).zip(1..10_000)),
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive unique sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).zip(1..10_000));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive repeated sequence,
            (
                {
                    let mut x = Vec::from_iter((0..10_000).zip(1..10_000).chain((0..10_000).zip(1..10_000)));
                    x.shuffle(&mut thread_rng());
                    x
                },
                (Vec::from_iter((0..10_000).zip(1..10_000)), 9_999),
            ),
            // ... very-long non-consecutive reversed sequence,
            (
                {
                    let mut x = Vec::from_iter(
                        (0..10_000)
                            .zip(1..10_000)
                            .chain((0..10_000).zip(1..10_000).map(|(x, y)| (y, x))),
                    );
                    x.shuffle(&mut thread_rng());
                    x
                },
                (
                    {
                        let mut x = Vec::from_iter(
                            (0..10_000)
                                .zip(1..10_000)
                                .chain((0..10_000).zip(1..10_000).map(|(x, y)| (y, x))),
                        );
                        x.sort_unstable();
                        x
                    },
                    19_998,
                ),
            ),
        ];

        // Test for each scenario.
        for (j, k) in data {
            let g = G::from_edges(j);
            let (e, s) = k;
            assert!(E!(g).eq(e.iter().map(|e| (&e.0, &e.1))));
            assert_eq!(g.size(), s);
        }
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
