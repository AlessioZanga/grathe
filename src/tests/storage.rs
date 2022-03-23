// TODO: Replace with is_sorted method on iterators once stable.
fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Storage;
    use crate::{Adj, E, V};
    use all_asserts::*;

    #[test]
    // FIXME:
    fn eq<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn partial_cmp<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    fn default<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::default() -> Self`.

        let g = G::default();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
    }

    #[test]
    fn new<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::new(I: IntoIter<Item = i32>, I: IntoIter<Item = (i32, i32)>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], (0, 0, vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], (1, 0, vec![&0], vec![])),
            // ... zero vertices and one edge,
            (vec![], vec![(0, 0)], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], (4, 0, vec![&0, &1, &2, &3], vec![])),
            // ... zero vertices and multiple edges,
            (
                vec![],
                vec![(0, 1), (1, 2), (2, 3)],
                (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
            ),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                (4, 1, vec![&0, &1, &2, &3], vec![(&0, &1)]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
            ),
            // ... random vertices and random edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    5,
                    4,
                    vec![&1, &3, &58, &71, &75],
                    vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
                ),
            ),
            // ... random non-overlapping vertices and edges,
            (
                vec![35, 62, 99, 29, 100, 18],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    11,
                    4,
                    vec![&1, &3, &18, &29, &35, &58, &62, &71, &75, &99, &100],
                    vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (o, s, v, e) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), s);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), e);
        }
    }

    #[test]
    fn null<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::null() -> Self`.

        let g = G::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
    }

    #[test]
    fn empty<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::empty(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], (0, vec![])),
            // ... one vertex,
            (vec![0], (1, vec![&0])),
            // ... multiple vertices,
            (vec![0, 1, 2, 3], (4, vec![&0, &1, &2, &3])),
            // ... random vertices,
            (vec![71, 1, 58, 3, 75], (5, vec![&1, &3, &58, &71, &75])),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::empty(i);
            let (o, v) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), 0);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
        }
    }

    #[test]
    fn complete<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::complete(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], (0, vec![], vec![])),
            // ... one vertex,
            (vec![0], (1, vec![&0], vec![(&0, &0)])),
            // ... multiple vertices,
            (
                vec![0, 1, 2, 3],
                (
                    4,
                    vec![&0, &1, &2, &3],
                    vec![
                        (&0, &0),
                        (&0, &1),
                        (&0, &2),
                        (&0, &3),
                        (&1, &1),
                        (&1, &2),
                        (&1, &3),
                        (&2, &2),
                        (&2, &3),
                        (&3, &3),
                    ],
                ),
            ),
            // ... random vertices and random edges,
            (
                vec![71, 1, 58, 3, 75],
                (
                    5,
                    vec![&1, &3, &58, &71, &75],
                    vec![
                        (&1, &1),
                        (&1, &3),
                        (&1, &58),
                        (&1, &71),
                        (&1, &75),
                        (&3, &3),
                        (&3, &58),
                        (&3, &71),
                        (&3, &75),
                        (&58, &58),
                        (&58, &71),
                        (&58, &75),
                        (&71, &71),
                        (&71, &75),
                        (&75, &75),
                    ],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::complete(i);
            let (o, v, e) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), (o * (o + 1)) / 2);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), e);
        }
    }

    #[test]
    fn clear<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::clear()`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)]),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![]),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)]),
            // ... multiple vertices and multiple edges,
            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
            // ... random vertices and random edges,
            (vec![71, 1, 58, 3, 75], vec![(71, 1), (1, 58), (58, 3), (3, 75)]),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, j);
            g.clear();
            assert_eq!(g.order(), 0);
            assert_eq!(g.size(), 0);
            assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
            assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
        }
    }

    #[test]
    fn vertices_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::vertices_iter() -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], vec![]),
            // ... one vertex,
            (vec![0], vec![&0]),
            // ... multiple vertices,
            (vec![0, 1, 2, 3], vec![&0, &1, &2, &3]),
            // ... random vertices,
            (vec![71, 1, 58, 3, 75], vec![&1, &3, &58, &71, &75]),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::new(i, vec![]);
            assert!(super::is_sorted(V!(g)));
            assert_eq!(Vec::from_iter(V!(g)), k);
        }
    }

    #[test]
    fn edges_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::edges_iter() -> Self`.

        // Test for ...
        let data = [
            // ... zero edges,
            (vec![], vec![]),
            // ... one edge,
            (vec![(0, 0)], vec![(&0, &0)]),
            // ... multiple edges,
            (vec![(0, 1), (1, 2), (2, 3)], vec![(&0, &1), (&1, &2), (&2, &3)]),
            // ... random edges,
            (
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
            ),
        ];

        // Test for each scenario.
        for (j, k) in data {
            let g = G::new(vec![], j);
            assert!(super::is_sorted(E!(g)));
            assert_eq!(Vec::from_iter(E!(g)), k);
        }
    }

    #[test]
    // FIXME:
    fn adjacents_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn adjacents_iter_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn order<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn size<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn has_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn add_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn del_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn has_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn has_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn add_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn add_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn del_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn del_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::Storage;
    use crate::{Adj, E, V};
    use all_asserts::*;

    #[test]
    // FIXME:
    fn eq<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn partial_cmp<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    fn default<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::default() -> Self`.

        let g = G::default();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
    }

    #[test]
    fn new<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::new(I: IntoIter<Item = i32>, I: IntoIter<Item = (i32, i32)>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], (0, 0, vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], (1, 0, vec![&0], vec![])),
            // ... zero vertices and one edge,
            (vec![], vec![(0, 0)], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], (4, 0, vec![&0, &1, &2, &3], vec![])),
            // ... zero vertices and multiple edges,
            (
                vec![],
                vec![(0, 1), (1, 2), (2, 3)],
                (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
            ),
            // ... random vertices and random edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    5,
                    4,
                    vec![&1, &3, &58, &71, &75],
                    vec![(&1, &58), (&3, &75), (&58, &3), (&71, &1)],
                ),
            ),
            // ... random non-overlapping vertices and edges,
            (
                vec![35, 62, 99, 29, 100, 18],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    11,
                    4,
                    vec![&1, &3, &18, &29, &35, &58, &62, &71, &75, &99, &100],
                    vec![(&1, &58), (&3, &75), (&58, &3), (&71, &1)],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (o, s, v, e) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), s);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), e);
        }
    }

    #[test]
    fn null<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::null() -> Self`.

        let g = G::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
    }

    #[test]
    fn empty<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::empty(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], (0, vec![])),
            // ... one vertex,
            (vec![0], (1, vec![&0])),
            // ... multiple vertices,
            (vec![0, 1, 2, 3], (4, vec![&0, &1, &2, &3])),
            // ... random vertices,
            (vec![71, 1, 58, 3, 75], (5, vec![&1, &3, &58, &71, &75])),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::empty(i);
            let (o, v) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), 0);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
        }
    }

    #[test]
    fn complete<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::complete(I: IntoIter<Item = i32>) -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], (0, vec![], vec![])),
            // ... one vertex,
            (vec![0], (1, vec![&0], vec![(&0, &0)])),
            // ... multiple vertices,
            (
                vec![0, 1, 2, 3],
                (
                    4,
                    vec![&0, &1, &2, &3],
                    vec![
                        (&0, &0),
                        (&0, &1),
                        (&0, &2),
                        (&0, &3),
                        (&1, &0),
                        (&1, &1),
                        (&1, &2),
                        (&1, &3),
                        (&2, &0),
                        (&2, &1),
                        (&2, &2),
                        (&2, &3),
                        (&3, &0),
                        (&3, &1),
                        (&3, &2),
                        (&3, &3),
                    ],
                ),
            ),
            // ... random vertices and random edges,
            (
                vec![71, 1, 58, 3, 75],
                (
                    5,
                    vec![&1, &3, &58, &71, &75],
                    vec![
                        (&1, &1),
                        (&1, &3),
                        (&1, &58),
                        (&1, &71),
                        (&1, &75),
                        (&3, &1),
                        (&3, &3),
                        (&3, &58),
                        (&3, &71),
                        (&3, &75),
                        (&58, &1),
                        (&58, &3),
                        (&58, &58),
                        (&58, &71),
                        (&58, &75),
                        (&71, &1),
                        (&71, &3),
                        (&71, &58),
                        (&71, &71),
                        (&71, &75),
                        (&75, &1),
                        (&75, &3),
                        (&75, &58),
                        (&75, &71),
                        (&75, &75),
                    ],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::complete(i);
            let (o, v, e) = k;
            assert_eq!(g.order(), o);
            assert_eq!(g.size(), o * o);
            assert_eq!(Vec::from_iter(V!(g)), v);
            assert_eq!(Vec::from_iter(E!(g)), e);
        }
    }

    #[test]
    fn clear<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::clear()`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)]),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![]),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)]),
            // ... multiple vertices and multiple edges,
            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
            // ... random vertices and random edges,
            (vec![71, 1, 58, 3, 75], vec![(71, 1), (1, 58), (58, 3), (3, 75)]),
        ];

        // Test for each scenario.
        for (i, j) in data {
            let mut g = G::new(i, j);
            g.clear();
            assert_eq!(g.order(), 0);
            assert_eq!(g.size(), 0);
            assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
            assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
        }
    }

    #[test]
    fn vertices_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::vertices_iter() -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices,
            (vec![], vec![]),
            // ... one vertex,
            (vec![0], vec![&0]),
            // ... multiple vertices,
            (vec![0, 1, 2, 3], vec![&0, &1, &2, &3]),
            // ... random vertices,
            (vec![71, 1, 58, 3, 75], vec![&1, &3, &58, &71, &75]),
        ];

        // Test for each scenario.
        for (i, k) in data {
            let g = G::empty(i);
            assert!(super::is_sorted(V!(g)));
            assert_eq!(Vec::from_iter(V!(g)), k);
        }
    }

    #[test]
    fn edges_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        // Test `G::edges_iter() -> Self`.

        // Test for ...
        let data = [
            // ... zero edges,
            (vec![], vec![]),
            // ... one edge,
            (vec![(0, 0)], vec![(&0, &0)]),
            // ... multiple edges,
            (vec![(0, 1), (1, 2), (2, 3)], vec![(&0, &1), (&1, &2), (&2, &3)]),
            // ... random edges,
            (
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![(&1, &58), (&3, &75), (&58, &3), (&71, &1)],
            ),
        ];

        // Test for each scenario.
        for (j, k) in data {
            let g = G::new(vec![], j);
            assert!(super::is_sorted(E!(g)));
            assert_eq!(Vec::from_iter(E!(g)), k);
        }
    }

    #[test]
    // FIXME:
    fn adjacents_iter<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn adjacents_iter_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn order<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn size<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn has_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn add_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn del_vertex<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    fn has_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn has_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn add_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn add_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[test]
    // FIXME:
    fn del_edge<G>()
    where
        G: Storage<Vertex = i32>,
    {
        todo!()
    }

    #[test]
    // FIXME:
    #[should_panic]
    fn del_edge_should_panic<G>()
    where
        G: Storage<Vertex = i32>,
    {
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
