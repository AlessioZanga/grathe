#[cfg(test)]
mod storage {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use std::cmp::Ordering;

                use grathe::{traits::Storage, utils::is_sorted, Adj, E, V};

                #[test]
                fn eq() {
                    // Test `G::eq(G) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (
                            vec![],
                            vec![],
                            vec![
                                (vec![], vec![], true),
                                (vec![0], vec![], false),
                                (vec![], vec![(0, 0)], false),
                            ],
                        ),
                        // ... one vertex and zero edges,
                        (
                            vec![0],
                            vec![],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], true),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], true),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], true),
                                (vec![], vec![(0, 0)], false),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], true),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], true),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                                (
                                    vec![71, 1, 58, 3, 75],
                                    vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                                    true,
                                ),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (i, j, f) in k {
                            let h = $G::<i32>::new(i, j);
                            assert_eq!(g.eq(&h), f);
                        }
                    }
                }

                #[test]
                fn partial_cmp() {
                    // Test `G::partial_cmp(G) -> Option<Ordering>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (
                            vec![],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Equal)),
                                (vec![0], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], Some(Ordering::Less)),
                            ],
                        ),
                        // ... one vertex and zero edges,
                        (
                            vec![0],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Equal)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], Some(Ordering::Less)),
                                (vec![], vec![(0, 1)], Some(Ordering::Less)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], None),
                                (vec![], vec![(0, 0)], Some(Ordering::Equal)),
                                (vec![], vec![(0, 1)], None),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Equal)),
                                (vec![0, 1, 2, 3, 4], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], None),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![71], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![71, 1], vec![], Some(Ordering::Greater)),
                                (vec![71, 1, 58, 3], vec![], Some(Ordering::Greater)),
                                (vec![71, 1, 58, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(71, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(71, 71)], None),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (i, j, f) in k {
                            let h = $G::<i32>::new(i, j);
                            assert!(g.partial_cmp(&h).eq(&f));
                        }
                    }
                }

                #[test]
                fn default() {
                    // Test `G::default() -> Self`.

                    let g = $G::<i32>::default();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                    assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                }

                #[test]
                fn new() {
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
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... zero vertices and multiple edges,
                        (
                            vec![],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                4,
                                3,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&1, &2), (&2, &3)],
                            ),
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
                            (
                                4,
                                3,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&1, &2), (&2, &3)],
                            ),
                        ),
                        // ... random vertices and edges,
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
                        let g = $G::<i32>::new(i, j);
                        let (o, s, v, e) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), s);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), e);
                    }
                }

                #[test]
                fn null() {
                    // Test `G::null() -> Self`.

                    let g = $G::<i32>::null();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                    assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                }

                #[test]
                fn empty() {
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
                        let g = $G::<i32>::empty(i);
                        let (o, v) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), 0);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                    }
                }

                #[test]
                fn complete() {
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
                        // ... random vertices and edges,
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
                        let g = $G::<i32>::complete(i);
                        let (o, v, e) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), (o * (o + 1)) / 2);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), e);
                    }
                }

                #[test]
                fn clear() {
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
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j) in data {
                        let mut g = $G::<i32>::new(i, j);
                        g.clear();
                        assert_eq!(g.order(), 0);
                        assert_eq!(g.size(), 0);
                        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                    }
                }

                #[test]
                fn vertices_iter() {
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
                        let g = $G::<i32>::new(i, vec![]);
                        assert!(is_sorted(V!(g)));
                        assert_eq!(Vec::from_iter(V!(g)), k);
                    }
                }

                #[test]
                fn edges_iter() {
                    // Test `G::edges_iter() -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero edges,
                        (vec![], vec![]),
                        // ... one edge,
                        (vec![(0, 0)], vec![(&0, &0)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(&0, &1), (&1, &2), (&2, &3)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new(vec![], j);
                        assert!(is_sorted(E!(g)));
                        assert_eq!(Vec::from_iter(E!(g)), k);
                    }
                }

                #[test]
                fn adjacents_iter() {
                    // Test `G::adjacents_iter(i32) -> Self`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![(&0, vec![])]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(&0, vec![&0])]),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![(&0, vec![]), (&1, vec![]), (&2, vec![]), (&3, vec![])],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![(&0, vec![&1]), (&1, vec![&0]), (&2, vec![]), (&3, vec![])],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (&0, vec![&1]),
                                (&1, vec![&0, &2]),
                                (&2, vec![&1, &3]),
                                (&3, vec![&2]),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (&1, vec![&58, &71]),
                                (&3, vec![&58, &75]),
                                (&58, vec![&1, &3]),
                                (&71, vec![&1]),
                                (&75, vec![&3]),
                            ],
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (&1, vec![&58, &71]),
                                (&3, vec![&58, &75]),
                                (&18, vec![]),
                                (&29, vec![]),
                                (&35, vec![]),
                                (&58, vec![&1, &3]),
                                (&62, vec![]),
                                (&71, vec![&1]),
                                (&75, vec![&3]),
                                (&99, vec![]),
                                (&100, vec![]),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (x, ys) in k {
                            assert!(is_sorted(Adj!(g, x)));
                            assert_eq!(Vec::from_iter(Adj!(g, x)), ys);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn adjacents_iter_should_panic() {
                    // Test `G::adjacents_iter(i32) -> Self`.

                    let g = $G::<i32>::null();
                    Adj!(g, &0);
                }

                #[test]
                fn order() {
                    // Test `G::order() -> usize`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], 0),
                        // ... one vertex,
                        (vec![0], 1),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], 4),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], 5),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let g = $G::<i32>::empty(i);
                        assert_eq!(g.order(), k);
                    }
                }

                #[test]
                fn size() {
                    // Test `G::size() -> usize`.

                    // Test for ...
                    let data = [
                        // ... zero edges,
                        (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], 1),
                        // ... multiple edges,
                        (vec![(0, 1), (1, 2), (2, 3)], 3),
                        // ... random edges,
                        (vec![(71, 1), (1, 58), (58, 3), (3, 75)], 4),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new([], j);
                        assert_eq!(g.size(), k);
                    }
                }

                #[test]
                fn has_vertex() {
                    // Test `G::has_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (&0, false)),
                        // ... one vertex,
                        (vec![0], (&0, true)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (&1, true)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (&2, false)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.has_vertex(v), f);
                    }
                }

                #[test]
                fn add_vertex() {
                    // Test `G::add_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, true)),
                        // ... one vertex,
                        (vec![0], (0, false)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (1, false)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (2, true)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let mut g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.add_vertex(v), f);
                    }
                }

                #[test]
                fn del_vertex() {
                    // Test `G::del_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (&0, false)),
                        // ... one vertex,
                        (vec![0], (&0, true)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (&1, true)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (&2, false)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let mut g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.del_vertex(v), f);
                    }
                }

                #[test]
                fn has_edge() {
                    // Test `G::has_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), true)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), true), ((&1, &0), true), ((&1, &3), false)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), true),
                                ((&1, &58), true),
                                ((&58, &1), true),
                                ((&71, &71), false),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.has_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn has_edge_should_panic() {
                    // Test `G::has_edge(i32, i32) -> bool`.

                    let g = $G::<i32>::null();
                    g.has_edge(&0, &0);
                }

                #[test]
                fn add_edge() {
                    // Test `G::add_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), false)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), false), ((&1, &0), false), ((&1, &3), true)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), false),
                                ((&1, &58), false),
                                ((&58, &1), false),
                                ((&71, &71), true),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let mut g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.add_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn add_edge_should_panic() {
                    // Test `G::add_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.add_edge(&0, &0);
                }

                #[test]
                fn del_edge() {
                    // Test `G::del_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), true)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), true), ((&1, &0), false), ((&1, &3), false)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), true),
                                ((&1, &58), true),
                                ((&58, &1), false),
                                ((&71, &71), false),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let mut g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.del_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn del_edge_should_panic() {
                    // Test `G::del_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.del_edge(&0, &0);
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
                use std::cmp::Ordering;

                use grathe::{traits::Storage, utils::is_sorted, Adj, E, V};

                #[test]
                fn eq() {
                    // Test `G::eq(G) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (
                            vec![],
                            vec![],
                            vec![
                                (vec![], vec![], true),
                                (vec![0], vec![], false),
                                (vec![], vec![(0, 0)], false),
                            ],
                        ),
                        // ... one vertex and zero edges,
                        (
                            vec![0],
                            vec![],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], true),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], true),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], true),
                                (vec![], vec![(0, 0)], false),
                                (vec![], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], true),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                            ],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], true),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (vec![], vec![], false),
                                (vec![0], vec![], false),
                                (vec![0, 1], vec![], false),
                                (vec![0, 1, 2, 3], vec![], false),
                                (vec![], vec![(0, 0)], false),
                                (vec![0, 1, 2, 3], vec![(0, 1)], false),
                                (vec![], vec![(0, 1), (1, 2), (2, 3)], false),
                                (
                                    vec![71, 1, 58, 3, 75],
                                    vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                                    true,
                                ),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (i, j, f) in k {
                            let h = $G::<i32>::new(i, j);
                            assert_eq!(g.eq(&h), f);
                        }
                    }
                }

                #[test]
                fn partial_cmp() {
                    // Test `G::partial_cmp(G) -> Option<Ordering>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (
                            vec![],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Equal)),
                                (vec![0], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], Some(Ordering::Less)),
                            ],
                        ),
                        // ... one vertex and zero edges,
                        (
                            vec![0],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Equal)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], Some(Ordering::Less)),
                                (vec![], vec![(0, 1)], Some(Ordering::Less)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], None),
                                (vec![], vec![(0, 0)], Some(Ordering::Equal)),
                                (vec![], vec![(0, 1)], None),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Equal)),
                                (vec![0, 1, 2, 3, 4], vec![], Some(Ordering::Less)),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], None),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![0], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![0, 1], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3], vec![], Some(Ordering::Greater)),
                                (vec![0, 1, 2, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(0, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(1, 1)], None),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (vec![], vec![], Some(Ordering::Greater)),
                                (vec![71], vec![], Some(Ordering::Greater)),
                                (vec![4], vec![], None),
                                (vec![71, 1], vec![], Some(Ordering::Greater)),
                                (vec![71, 1, 58, 3], vec![], Some(Ordering::Greater)),
                                (vec![71, 1, 58, 3, 4], vec![], None),
                                (vec![], vec![(0, 0)], None),
                                (vec![], vec![(71, 1)], Some(Ordering::Greater)),
                                (vec![], vec![(71, 71)], None),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (i, j, f) in k {
                            let h = $G::<i32>::new(i, j);
                            assert!(g.partial_cmp(&h).eq(&f));
                        }
                    }
                }

                #[test]
                fn default() {
                    // Test `G::default() -> Self`.

                    let g = $G::<i32>::default();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                    assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                }

                #[test]
                fn new() {
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
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            (4, 0, vec![&0, &1, &2, &3], vec![]),
                        ),
                        // ... zero vertices and multiple edges,
                        (
                            vec![],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                4,
                                3,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&1, &2), (&2, &3)],
                            ),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                4,
                                3,
                                vec![&0, &1, &2, &3],
                                vec![(&0, &1), (&1, &2), (&2, &3)],
                            ),
                        ),
                        // ... random vertices and edges,
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
                        let g = $G::<i32>::new(i, j);
                        let (o, s, v, e) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), s);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), e);
                    }
                }

                #[test]
                fn null() {
                    // Test `G::null() -> Self`.

                    let g = $G::<i32>::null();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                    assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                }

                #[test]
                fn empty() {
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
                        let g = $G::<i32>::empty(i);
                        let (o, v) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), 0);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                    }
                }

                #[test]
                fn complete() {
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
                        // ... random vertices and edges,
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
                        let g = $G::<i32>::complete(i);
                        let (o, v, e) = k;
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), o * o);
                        assert_eq!(Vec::from_iter(V!(g)), v);
                        assert_eq!(Vec::from_iter(E!(g)), e);
                    }
                }

                #[test]
                fn clear() {
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
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j) in data {
                        let mut g = $G::<i32>::new(i, j);
                        g.clear();
                        assert_eq!(g.order(), 0);
                        assert_eq!(g.size(), 0);
                        assert_eq!(Vec::from_iter(V!(g)), Vec::<&i32>::new());
                        assert_eq!(Vec::from_iter(E!(g)), Vec::<(&i32, &i32)>::new());
                    }
                }

                #[test]
                fn vertices_iter() {
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
                        let g = $G::<i32>::empty(i);
                        assert!(is_sorted(V!(g)));
                        assert_eq!(Vec::from_iter(V!(g)), k);
                    }
                }

                #[test]
                fn edges_iter() {
                    // Test `G::edges_iter() -> Self`.

                    // Test for ...
                    let data = [
                        // ... zero edges,
                        (vec![], vec![]),
                        // ... one edge,
                        (vec![(0, 0)], vec![(&0, &0)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(&0, &1), (&1, &2), (&2, &3)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![(&1, &58), (&3, &75), (&58, &3), (&71, &1)],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new(vec![], j);
                        assert!(is_sorted(E!(g)));
                        assert_eq!(Vec::from_iter(E!(g)), k);
                    }
                }

                #[test]
                fn adjacents_iter() {
                    // Test `G::adjacents_iter(i32) -> Self`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero vertices and zero edges,
                        // (vec![], vec![], (0, 0, vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![(&0, vec![])]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(&0, vec![&0])]),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![(&0, vec![]), (&1, vec![]), (&2, vec![]), (&3, vec![])],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![(&0, vec![&1]), (&1, vec![&0]), (&2, vec![]), (&3, vec![])],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![
                                (&0, vec![&1]),
                                (&1, vec![&0, &2]),
                                (&2, vec![&1, &3]),
                                (&3, vec![&2]),
                            ],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (&1, vec![&58, &71]),
                                (&3, vec![&58, &75]),
                                (&58, vec![&1, &3]),
                                (&71, vec![&1]),
                                (&75, vec![&3]),
                            ],
                        ),
                        // ... random non-overlapping vertices and edges,
                        (
                            vec![35, 62, 99, 29, 100, 18],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (&1, vec![&58, &71]),
                                (&3, vec![&58, &75]),
                                (&18, vec![]),
                                (&29, vec![]),
                                (&35, vec![]),
                                (&58, vec![&1, &3]),
                                (&62, vec![]),
                                (&71, vec![&1]),
                                (&75, vec![&3]),
                                (&99, vec![]),
                                (&100, vec![]),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i, j);
                        for (x, ys) in k {
                            assert!(is_sorted(Adj!(g, x)));
                            assert_eq!(Vec::from_iter(Adj!(g, x)), ys);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn adjacents_iter_should_panic() {
                    // Test `G::adjacents_iter(i32) -> Self`.

                    let g = $G::<i32>::null();
                    Adj!(g, &0);
                }

                #[test]
                fn order() {
                    // Test `G::order() -> usize`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], 0),
                        // ... one vertex,
                        (vec![0], 1),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], 4),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], 5),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let g = $G::<i32>::empty(i);
                        assert_eq!(g.order(), k);
                    }
                }

                #[test]
                fn size() {
                    // Test `G::size() -> usize`.

                    // Test for ...
                    let data = [
                        // ... zero edges,
                        (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], 1),
                        // ... multiple edges,
                        (vec![(0, 1), (1, 2), (2, 3)], 3),
                        // ... random edges,
                        (vec![(71, 1), (1, 58), (58, 3), (3, 75)], 4),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new([], j);
                        assert_eq!(g.size(), k);
                    }
                }

                #[test]
                fn has_vertex() {
                    // Test `G::has_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (&0, false)),
                        // ... one vertex,
                        (vec![0], (&0, true)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (&1, true)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (&2, false)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.has_vertex(v), f);
                    }
                }

                #[test]
                fn add_vertex() {
                    // Test `G::add_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, true)),
                        // ... one vertex,
                        (vec![0], (0, false)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (1, false)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (2, true)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let mut g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.add_vertex(v), f);
                    }
                }

                #[test]
                fn del_vertex() {
                    // Test `G::del_vertex(i32) -> bool`.

                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (&0, false)),
                        // ... one vertex,
                        (vec![0], (&0, true)),
                        // ... multiple vertices,
                        (vec![0, 1, 2, 3], (&1, true)),
                        // ... random vertices,
                        (vec![71, 1, 58, 3, 75], (&2, false)),
                    ];

                    // Test for each scenario.
                    for (i, k) in data {
                        let mut g = $G::<i32>::empty(i);
                        let (v, f) = k;
                        assert_eq!(g.del_vertex(v), f);
                    }
                }

                #[test]
                fn has_edge() {
                    // Test `G::has_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), true)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), true), ((&1, &0), false), ((&1, &3), false)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), true),
                                ((&1, &58), true),
                                ((&58, &1), false),
                                ((&71, &71), false),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.has_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn has_edge_should_panic() {
                    // Test `G::has_edge(i32, i32) -> bool`.

                    let g = $G::<i32>::null();
                    g.has_edge(&0, &0);
                }

                #[test]
                fn add_edge() {
                    // Test `G::add_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), false)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), false), ((&1, &0), true), ((&1, &3), true)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), false),
                                ((&1, &58), false),
                                ((&58, &1), true),
                                ((&71, &71), true),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let mut g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.add_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn add_edge_should_panic() {
                    // Test `G::add_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.add_edge(&0, &0);
                }

                #[test]
                fn del_edge() {
                    // Test `G::del_edge(i32, i32) -> bool`.

                    // Test for ...
                    let data = [
                        // NOTE: This would panic!
                        // ... zero edges,
                        // (vec![], 0),
                        // ... one edge,
                        (vec![(0, 0)], vec![((&0, &0), true)]),
                        // ... multiple edges,
                        (
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![((&0, &1), true), ((&1, &0), false), ((&1, &3), false)],
                        ),
                        // ... random edges,
                        (
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                ((&71, &1), true),
                                ((&1, &58), true),
                                ((&58, &1), false),
                                ((&71, &71), false),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (j, k) in data {
                        let mut g = $G::<i32>::new([], j);
                        for ((x, y), f) in k {
                            assert_eq!(g.del_edge(x, y), f);
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn del_edge_should_panic() {
                    // Test `G::del_edge(i32, i32) -> bool`.

                    let mut g = $G::<i32>::null();
                    g.del_edge(&0, &0);
                }
            };
        }

        mod directed_adjacency_list {
            use grathe::graphs::storages::DirectedAdjacencyList;
            generic_tests!(DirectedAdjacencyList);
        }
    }
}
