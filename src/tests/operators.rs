#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::{Operators, Storage};
    use crate::{E, V};
    use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};

    #[test]
    fn complement<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: Not<Output = G>,
    {
        // Test `G::complement() -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], (0, 0, vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], (1, 0, vec![&0], vec![])),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                (
                    4,
                    10,
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
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                (
                    4,
                    9,
                    vec![&0, &1, &2, &3],
                    vec![
                        (&0, &0),
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
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (
                    4,
                    7,
                    vec![&0, &1, &2, &3],
                    vec![(&0, &0), (&0, &2), (&0, &3), (&1, &1), (&1, &3), (&2, &2), (&3, &3)],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let h = g.complement();
            assert_eq!(h, !&g);
            assert_eq!(g, !&h); // G = !!G.

            let (o, s, v, e) = k;
            assert_eq!(h.order(), o);
            assert_eq!(h.size(), s);
            assert_eq!(Vec::from_iter(V!(h)), v);
            assert_eq!(Vec::from_iter(E!(h)), e);
        }
    }

    #[test]
    #[ignore]
    // FIXME:
    fn union<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitOr<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn intersection<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitAnd<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn symmetric_difference<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitXor<&'a G, Output = G> + Sub<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn difference<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: Sub<&'a G, Output = G>,
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
    use crate::traits::{Operators, Storage};
    use crate::{E, V};
    use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};

    #[test]
    fn complement<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: Not<Output = G>,
    {
        // Test `G::complement() -> Self`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], (0, 0, vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], (1, 1, vec![&0], vec![(&0, &0)])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], (1, 0, vec![&0], vec![])),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                (
                    4,
                    16,
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
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                (
                    4,
                    15,
                    vec![&0, &1, &2, &3],
                    vec![
                        (&0, &0),
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
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (
                    4,
                    13,
                    vec![&0, &1, &2, &3],
                    vec![
                        (&0, &0),
                        (&0, &2),
                        (&0, &3),
                        (&1, &0),
                        (&1, &1),
                        (&1, &3),
                        (&2, &0),
                        (&2, &1),
                        (&2, &2),
                        (&3, &0),
                        (&3, &1),
                        (&3, &2),
                        (&3, &3),
                    ],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            println!("{:?}, {:?}", i.iter(), j.iter());
            let g = G::new(i, j);
            let h = g.complement();
            assert_eq!(h, !&g);
            assert_eq!(g, !&h); // G = !!G.

            let (o, s, v, e) = k;
            assert_eq!(h.order(), o);
            assert_eq!(h.size(), s);
            assert_eq!(Vec::from_iter(V!(h)), v);
            assert_eq!(Vec::from_iter(E!(h)), e);
        }
    }

    #[test]
    #[ignore]
    // FIXME:
    fn union<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitOr<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn intersection<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitAnd<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn symmetric_difference<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: BitXor<&'a G, Output = G> + Sub<&'a G, Output = G>,
    {
        todo!()
    }

    #[test]
    #[ignore]
    // FIXME:
    fn difference<G>()
    where
        G: Storage<Vertex = i32> + Operators,
        for<'a> &'a G: Sub<&'a G, Output = G>,
    {
        todo!()
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
