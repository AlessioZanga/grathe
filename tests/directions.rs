#[cfg(test)]
mod directions {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use grathe::{
                    traits::{Storage, Undirected},
                    utils::is_sorted,
                    Ne,
                };

                #[test]
                fn neighbors_iter() {
                    // Test `G::neighbors_iter(i32) -> Iter<Item = i32>`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, vec![&0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&0, vec![&1]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&58, &71]),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&58, &71]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(Vec::from_iter(Ne!(g, x)), l);
                        assert!(is_sorted(Ne!(g, x)));
                    }
                }

                #[test]
                #[should_panic]
                fn neighbors_iter_should_panic() {
                    // Test `G::neighbors_iter(i32) -> Iter<Item = i32>`.

                    let g = $G::<i32>::null();
                    Ne!(g, &0);
                }

                #[test]
                fn add_undirected_edge() {
                    // Test `G::add_undirected_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, &0, true)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, &0, false)),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, &0, true)),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&0, &1, false),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&58, &3, false),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&62, &99, true),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let mut g = $G::<i32>::new(i, j);
                        let (x, y, f) = k;
                        assert_eq!(g.add_undirected_edge(x, y), f);
                    }
                }

                #[test]
                #[should_panic]
                fn add_undirected_edge_should_panic() {
                    // Test `G::add_undirected_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.add_undirected_edge(&0, &0);
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
                use grathe::{
                    traits::{Directed, Storage},
                    utils::is_sorted,
                    An, Ch, De, Pa,
                };

                #[test]
                fn ancestors_iter() {
                    // Test `G::ancestors_iter(i32) -> Iter<Item = i32>`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, vec![&0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&3, vec![&0, &1, &2]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&71]),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&71]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(Vec::from_iter(An!(g, x)), l);
                        assert!(is_sorted(An!(g, x)));
                    }
                }

                #[test]
                #[should_panic]
                fn ancestors_iter_should_panic() {
                    // Test `G::ancestors_iter(i32) -> Iter<Item = i32>`.

                    let g = $G::<i32>::null();
                    An!(g, &0);
                }

                #[test]
                fn parents_iter() {
                    // Test `G::parents_iter(i32) -> Iter<Item = i32>`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, vec![&0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&3, vec![&2]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&71]),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&71]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(Vec::from_iter(Pa!(g, x)), l);
                        assert!(is_sorted(Pa!(g, x)));
                    }
                }

                #[test]
                #[should_panic]
                fn parents_iter_should_panic() {
                    // Test `G::parents_iter(i32) -> Iter<Item = i32>`.

                    let g = $G::<i32>::null();
                    Pa!(g, &0);
                }

                #[test]
                fn children_iter() {
                    // Test `G::children_iter(i32) -> Iter<Item = i32>`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, vec![&0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&2, vec![&3]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&71, vec![&1]),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&71, vec![&1]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(Vec::from_iter(Ch!(g, x)), l);
                        assert!(is_sorted(Ch!(g, x)));
                    }
                }

                #[test]
                #[should_panic]
                fn children_iter_should_panic() {
                    // Test `G::children_iter(i32) -> Iter<Item = i32>`.

                    let g = $G::<i32>::null();
                    Ch!(g, &0);
                }

                #[test]
                fn descendants_iter() {
                    // Test `G::children_iter(i32) -> Iter<Item = i32>`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, vec![&0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&0, vec![&1, &2, &3]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&3, &58, &75]),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, vec![&3, &58, &75]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(Vec::from_iter(De!(g, x)), l);
                        assert!(is_sorted(De!(g, x)));
                    }
                }

                #[test]
                #[should_panic]
                fn descendants_iter_should_panic() {
                    // Test `G::descendants_iter(i32) -> Iter<Item = i32>`.

                    let g = $G::<i32>::null();
                    De!(g, &0);
                }

                #[test]
                fn add_directed_edge() {
                    // Test `G::add_directed_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, &0, true)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, &0, false)),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, &0, true)),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (&0, &1, false),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&3, &58, true),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&62, &99, true),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let mut g = $G::<i32>::new(i, j);
                        let (x, y, f) = k;
                        assert_eq!(g.add_directed_edge(x, y), f);
                    }
                }

                #[test]
                #[should_panic]
                fn add_directed_edge_should_panic() {
                    // Test `G::add_directed_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.add_directed_edge(&0, &0);
                }

                #[test]
                fn in_degree() {
                    // Test `G::in_degree(i32) -> usize`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, 0)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, 1)),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, 0)),
                        // ... multiple vertices and multiple edges,
                        (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)], (&1, 1)),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, 1),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&3, 1),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(g.in_degree(x), l);
                        assert_eq!(g.in_degree(x), Pa!(g, x).count());
                    }
                }

                #[test]
                #[should_panic]
                fn in_degree_should_panic() {
                    // Test `G::in_degree(i32) -> usize`.

                    let g = $G::<i32>::null();
                    g.in_degree(&0);
                }

                #[test]
                fn out_degree() {
                    // Test `G::out_degree(i32) -> usize`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], (&0, vec![]))),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], (&0, 0)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], (&0, 1)),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], (&0, 0)),
                        // ... multiple vertices and multiple edges,
                        (vec![0, 1, 2, 3], vec![(0, 1), (1, 2), (2, 3)], (&1, 1)),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&1, 1),
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (&3, 1),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        let (x, l) = k;
                        assert_eq!(g.out_degree(x), l);
                        assert_eq!(g.out_degree(x), Ch!(g, x).count());
                    }
                }

                #[test]
                #[should_panic]
                fn out_degree_should_panic() {
                    // Test `G::out_degree(i32) -> usize`.

                    let g = $G::<i32>::null();
                    g.out_degree(&0);
                }
            };
        }

        mod directed_adjacency_list {
            use grathe::graphs::storages::DirectedAdjacencyList;
            generic_tests!(DirectedAdjacencyList);
        }
    }
}