#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            paste::item! {
                use grathe::traits::{Operators, Storage};
                use grathe::{E, V};

                #[test]
                fn complement()
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
                        let g = $G::<i32>::new(i, j);
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
                fn union()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn intersection()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn symmetric_difference()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn difference()
                {
                    todo!()
                }
            }
        };
    }

    mod undirected_adjacency_list {
        use grathe::graphs::storages::UndirectedAdjacencyList;
        generic_tests!(UndirectedAdjacencyList);
    }
}

#[cfg(test)]
mod directed {
    macro_rules! generic_tests {
        ($G: ident) => {
            paste::item! {
                use grathe::traits::{Operators, Storage};
                use grathe::{E, V};

                #[test]
                fn complement()
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
                        let g = $G::<i32>::new(i, j);
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
                fn union()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn intersection()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn symmetric_difference()
                {
                    todo!()
                }

                #[test]
                #[ignore]
                // FIXME:
                fn difference()
                {
                    todo!()
                }
            }
        };
    }

    mod directed_adjacency_list {
        use grathe::graphs::storages::DirectedAdjacencyList;
        generic_tests!(DirectedAdjacencyList);
    }
}
