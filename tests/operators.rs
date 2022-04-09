#[cfg(test)]
mod operators {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use grathe::traits::{Operators, Storage};
                use grathe::{E, V};

                #[test]
                fn complement() {
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
                                vec![
                                    (&0, &0),
                                    (&0, &2),
                                    (&0, &3),
                                    (&1, &1),
                                    (&1, &3),
                                    (&2, &2),
                                    (&3, &3),
                                ],
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
                fn union() {
                    // Test `G::union(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (
                            (vec![], vec![]),
                            (vec![0], vec![]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 1, vec![&0], vec![(&0, &0)]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1], vec![]),
                            (vec![2, 3], vec![]),
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1], vec![(0, 1)]),
                            (vec![0, 1, 2, 3], vec![(1, 0)]),
                            (4, 1, vec![&0, &1, &2, &3], vec![(&0, &1)]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2)]),
                            (vec![0, 1, 2, 3], vec![(0, 3), (2, 3)]),
                            (
                                4,
                                4,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&0, &3), (&1, &2), (&2, &3)],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.union(&g);
                        assert_eq!(h, (&f | &g));
                        assert_eq!(h, (&g | &f));
                        assert_eq!(h, (&f | &h));
                        assert_eq!(h, (&g | &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn intersection() {
                    // Test `G::intersection(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        ((vec![], vec![]), (vec![0], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (2, 0, vec![&0, &1], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (2, 1, vec![&0, &1], vec![(&0, &1)]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 2, vec![&0, &1, &2, &3], vec![(&0, &1), (&2, &3)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.intersection(&g);
                        assert_eq!(h, (&f & &g));
                        assert_eq!(h, (&g & &f));
                        assert_eq!(h, (&f & &h));
                        assert_eq!(h, (&g & &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn symmetric_difference() {
                    // Test `G::symmetric_difference(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (
                            (vec![], vec![]),
                            (vec![0], vec![]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 1, vec![&0], vec![(&0, &0)]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (6, 0, vec![&0, &1, &2, &3, &4, &5], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (6, 0, vec![&0, &1, &2, &3, &4, &5], vec![]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 2, vec![&0, &1, &2, &3], vec![(&1, &2), (&3, &3)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.symmetric_difference(&g);
                        assert_eq!(h, (&f ^ &g));
                        assert_eq!(h, (&g ^ &f));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn difference() {
                    // Test `G::difference(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        ((vec![], vec![]), (vec![0], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 1, vec![&0, &1, &2, &3], vec![(&1, &2)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.difference(&g);
                        assert_eq!(h, (&f - &g));
                        // assert_eq!(h, (&g - &f));
                        // assert_eq!(h, (&f - &h));
                        // assert_eq!(h, (&g - &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }
            };
        }

        mod undirected_adjacency_list {
            use grathe::graphs::storages::UndirectedAdjacencyList;
            generic_tests!(UndirectedAdjacencyList);
        }
    }

    mod directed {
        macro_rules! generic_tests {
            ($G: ident) => {
                use grathe::traits::{Operators, Storage};
                use grathe::{E, V};

                #[test]
                fn complement() {
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
                fn union() {
                    // Test `G::union(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (
                            (vec![], vec![]),
                            (vec![0], vec![]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 1, vec![&0], vec![(&0, &0)]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1], vec![]),
                            (vec![2, 3], vec![]),
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1], vec![(0, 1)]),
                            (vec![0, 1, 2, 3], vec![(1, 0)]),
                            (4, 2, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &0)]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2)]),
                            (vec![0, 1, 2, 3], vec![(0, 3), (2, 3)]),
                            (
                                4,
                                4,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&0, &3), (&1, &2), (&2, &3)],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        println!("({:?}, {:?}), ({:?}, {:?})", a, b, i, j);
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.union(&g);
                        assert_eq!(h, (&f | &g));
                        assert_eq!(h, (&g | &f));
                        assert_eq!(h, (&f | &h));
                        assert_eq!(h, (&g | &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn intersection() {
                    // Test `G::intersection(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        ((vec![], vec![]), (vec![0], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (2, 0, vec![&0, &1], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (2, 0, vec![&0, &1], vec![]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 2, vec![&0, &1, &2, &3], vec![(&0, &1), (&2, &3)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.intersection(&g);
                        assert_eq!(h, (&f & &g));
                        assert_eq!(h, (&g & &f));
                        assert_eq!(h, (&f & &h));
                        assert_eq!(h, (&g & &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn symmetric_difference() {
                    // Test `G::symmetric_difference(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (
                            (vec![], vec![]),
                            (vec![0], vec![]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 1, vec![&0], vec![(&0, &0)]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (6, 0, vec![&0, &1, &2, &3, &4, &5], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (6, 2, vec![&0, &1, &2, &3, &4, &5], vec![(&0, &1), (&1, &0)]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 2, vec![&0, &1, &2, &3], vec![(&1, &2), (&3, &3)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.symmetric_difference(&g);
                        assert_eq!(h, (&f ^ &g));
                        assert_eq!(h, (&g ^ &f));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }

                #[test]
                fn difference() {
                    // Test `G::difference(&G) -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        ((vec![], vec![]), (vec![], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        ((vec![], vec![]), (vec![0], vec![]), (0, 0, vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            (vec![0], vec![]),
                            (vec![0], vec![(0, 0)]),
                            (1, 0, vec![&0], vec![]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            (vec![0, 1, 2, 3], vec![]),
                            (vec![0, 1, 4, 5], vec![]),
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1)]),
                            (vec![0, 1, 4, 5], vec![(1, 0)]),
                            (4, 1, vec![&0, &1, &2, &3], vec![(&0, &1)]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)]),
                            (vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (3, 3)]),
                            (4, 1, vec![&0, &1, &2, &3], vec![(&1, &2)]),
                        ),
                    ];

                    // Test for each scenario.
                    for ((a, b), (i, j), k) in data {
                        let f = $G::<i32>::new(a, b);
                        let g = $G::<i32>::new(i, j);
                        let h = f.difference(&g);
                        assert_eq!(h, (&f - &g));
                        // assert_eq!(h, (&g - &f));
                        // assert_eq!(h, (&f - &h));
                        // assert_eq!(h, (&g - &h));

                        let (o, s, v, e) = k;
                        assert_eq!(h.order(), o);
                        assert_eq!(h.size(), s);
                        assert_eq!(Vec::from_iter(V!(h)), v);
                        assert_eq!(Vec::from_iter(E!(h)), e);
                    }
                }
            };
        }

        mod directed_adjacency_list {
            use grathe::graphs::storages::DirectedAdjacencyList;
            generic_tests!(DirectedAdjacencyList);
        }
    }
}
