#[cfg(test)]
mod with_attributes {
    macro_rules! generic_tests {
        ($G: ident) => {
            use grathe::{
                graphs::attributes::AttributesMap,
                traits::{Storage, WithAttributes},
                E, V,
            };

            #[test]
            fn new_with_attributes() {
                // Test `G::new_with_attributes(
                //      Option<bool>,
                //      Iter<Item = (i32, (i64, ))>,
                //      Iter<Item = ((i32, i32), (f64, ))>
                //  ) -> bool`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (None, vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], (1, 0, vec![&0], vec![])),
                    // ... zero vertices and one edge,
                    (
                        Some((true,)),
                        vec![],
                        vec![((0, 0), (0.,))],
                        (1, 1, vec![&0], vec![(&0, &0)]),
                    ),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        (1, 1, vec![&0], vec![(&0, &0)]),
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![],
                        (4, 0, vec![&0, &1, &2, &3], vec![]),
                    ),
                    // ... zero vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,))],
                        (4, 1, vec![&0, &1, &2, &3], vec![(&0, &1)]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        (4, 3, vec![&0, &1, &2, &3], vec![(&0, &1), (&1, &2), (&2, &3)]),
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        (
                            5,
                            4,
                            vec![&1, &3, &58, &71, &75],
                            vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
                        ),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        (
                            11,
                            4,
                            vec![&1, &3, &18, &29, &35, &58, &62, &71, &75, &99, &100],
                            vec![(&1, &58), (&1, &71), (&3, &58), (&3, &75)],
                        ),
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    let (o, s, v, e) = l;
                    assert_eq!(g.order(), o);
                    assert_eq!(g.size(), s);
                    assert_eq!(Vec::from_iter(V!(g)), v);
                    assert_eq!(Vec::from_iter(E!(g)), e);
                }
            }

            #[test]
            fn has_graph_attrs() {
                // Test `G::has_graph_attrs() -> bool`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (None, vec![], vec![], false),
                    // ... one vertex and one edge,
                    (Some((true,)), vec![(0, (0,))], vec![((0, 0), (0.,))], true),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    assert_eq!(g.has_graph_attrs(), l);
                }
            }

            #[test]
            fn get_graph_attrs() {
                // Test `G::get_graph_attrs() -> Result`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (None, vec![], vec![], None),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        Some(&(true,)),
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    assert_eq!(g.get_graph_attrs().ok(), l);
                }
            }

            #[test]
            fn get_mut_graph_attrs() {
                // Test `G::get_mut_graph_attrs() -> Result`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (None, vec![], vec![], None),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        Some(&(true,)),
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    assert_eq!(g.get_mut_graph_attrs().ok().copied().as_ref(), l);
                }
            }

            #[test]
            fn set_graph_attrs() {
                // Test `G::set_graph_attrs((bool, ))`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    ((true,), vec![], vec![], Some(&(true,))),
                    // ... one vertex and one edge,
                    ((true,), vec![(0, (0,))], vec![((0, 0), (0.,))], Some(&(true,))),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(None, j, k);
                    assert!(!g.has_graph_attrs());
                    g.set_graph_attrs(i);
                    assert!(g.has_graph_attrs());
                    assert_eq!(g.get_graph_attrs().ok(), l);
                }
            }

            #[test]
            fn unset_graph_attrs() {
                // Test `G::unset_graph_attrs() -> Result`.

                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (Some((true,)), vec![], vec![], Some(&(true,))),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        Some(&(true,)),
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    assert!(g.has_graph_attrs());
                    assert_eq!(g.unset_graph_attrs().ok().as_ref(), l);
                    assert!(!g.has_graph_attrs());
                }
            }

            #[test]
            fn has_vertex_attrs() {
                // Test `G::has_vertex_attrs(i32) -> bool`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![(&0, true)]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![(&0, true)],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (1.,))],
                        vec![(&0, true), (&1, false), (&2, true), (&3, true)],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (58, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (1.,)),
                            ((1, 58), (1.,)),
                            ((58, 3), (1.,)),
                            ((3, 75), (1.,)),
                        ],
                        vec![(&1, false), (&3, false), (&58, true), (&71, true), (&75, true)],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            (&1, false),
                            (&3, false),
                            (&18, true),
                            (&29, true),
                            (&35, true),
                            (&58, false),
                            (&62, true),
                            (&71, false),
                            (&75, false),
                            (&99, true),
                            (&100, true),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for (x, f) in l {
                        assert_eq!(g.has_vertex_attrs(x), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn has_vertex_attrs_should_panic() {
                // Test `G::has_vertex_attrs() -> bool`.

                let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.has_vertex_attrs(&0);
            }

            #[test]
            fn get_vertex_attrs() {
                // Test `G::get_vertex_attrs(i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![(&0, Some(&(0,)))]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![(&0, Some(&(0,)))],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (1.,))],
                        vec![
                            (&0, Some(&(0,))),
                            (&1, None),
                            (&2, Some(&(0,))),
                            (&3, Some(&(0,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (58, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (1.,)),
                            ((1, 58), (1.,)),
                            ((58, 3), (1.,)),
                            ((3, 75), (1.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&58, Some(&(0,))),
                            (&71, Some(&(0,))),
                            (&75, Some(&(0,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&18, Some(&(0,))),
                            (&29, Some(&(0,))),
                            (&35, Some(&(0,))),
                            (&58, None),
                            (&62, Some(&(0,))),
                            (&71, None),
                            (&75, None),
                            (&99, Some(&(0,))),
                            (&100, Some(&(0,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for (x, f) in l {
                        assert_eq!(g.get_vertex_attrs(x).ok(), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn get_vertex_attrs_should_panic() {
                // Test `G::get_vertex_attrs(i32) -> Result`.

                let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.get_vertex_attrs(&0).unwrap();
            }

            #[test]
            fn get_mut_vertex_attrs() {
                // Test `G::get_vertex_attrs(i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![(&0, Some(&(0,)))]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![(&0, Some(&(0,)))],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (1.,))],
                        vec![
                            (&0, Some(&(0,))),
                            (&1, None),
                            (&2, Some(&(0,))),
                            (&3, Some(&(0,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (58, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (1.,)),
                            ((1, 58), (1.,)),
                            ((58, 3), (1.,)),
                            ((3, 75), (1.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&58, Some(&(0,))),
                            (&71, Some(&(0,))),
                            (&75, Some(&(0,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&18, Some(&(0,))),
                            (&29, Some(&(0,))),
                            (&35, Some(&(0,))),
                            (&58, None),
                            (&62, Some(&(0,))),
                            (&71, None),
                            (&75, None),
                            (&99, Some(&(0,))),
                            (&100, Some(&(0,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for (x, f) in l {
                        assert_eq!(g.get_mut_vertex_attrs(x).ok().copied().as_ref(), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn get_mut_vertex_attrs_should_panic() {
                // Test `G::get_mut_vertex_attrs(i32) -> Result`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.get_mut_vertex_attrs(&0).unwrap();
            }

            #[test]
            fn set_vertex_attrs() {
                // Test `G::set_vertex_attrs(i32, (i64,))`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![(&0, Some(&(0,)))]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![(&0, Some(&(0,)))],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (1.,))],
                        vec![
                            (&0, Some(&(0,))),
                            (&1, None),
                            (&2, Some(&(0,))),
                            (&3, Some(&(0,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (58, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (1.,)),
                            ((1, 58), (1.,)),
                            ((58, 3), (1.,)),
                            ((3, 75), (1.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&58, Some(&(0,))),
                            (&71, Some(&(0,))),
                            (&75, Some(&(0,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&18, Some(&(0,))),
                            (&29, Some(&(0,))),
                            (&35, Some(&(0,))),
                            (&58, None),
                            (&62, Some(&(0,))),
                            (&71, None),
                            (&75, None),
                            (&99, Some(&(0,))),
                            (&100, Some(&(0,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for (x, f) in l {
                        assert_eq!(g.has_vertex_attrs(x), f.is_some());
                        assert_eq!(g.unset_vertex_attrs(x).ok().as_ref(), f);
                        assert!(!g.has_vertex_attrs(x));
                        if let Some(f) = f {
                            g.set_vertex_attrs(x, f.clone());
                            assert!(g.has_vertex_attrs(x));
                            assert_eq!(g.get_vertex_attrs(x).ok(), Some(f));
                        }
                    }
                }
            }

            #[test]
            #[should_panic]
            fn set_vertex_attrs_should_panic() {
                // Test `G::set_vertex_attrs(i32, (i64, ))`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.set_vertex_attrs(&0, (1,));
            }

            #[test]
            fn unset_vertex_attrs() {
                // Test `G::unset_vertex_attrs(i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![(&0, Some(&(0,)))]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![(&0, Some(&(0,)))],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (1.,))],
                        vec![
                            (&0, Some(&(0,))),
                            (&1, None),
                            (&2, Some(&(0,))),
                            (&3, Some(&(0,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (58, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (1.,)),
                            ((1, 58), (1.,)),
                            ((58, 3), (1.,)),
                            ((3, 75), (1.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&58, Some(&(0,))),
                            (&71, Some(&(0,))),
                            (&75, Some(&(0,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            (&1, None),
                            (&3, None),
                            (&18, Some(&(0,))),
                            (&29, Some(&(0,))),
                            (&35, Some(&(0,))),
                            (&58, None),
                            (&62, Some(&(0,))),
                            (&71, None),
                            (&75, None),
                            (&99, Some(&(0,))),
                            (&100, Some(&(0,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for (x, f) in l {
                        assert_eq!(g.has_vertex_attrs(x), f.is_some());
                        assert_eq!(g.unset_vertex_attrs(x).ok().as_ref(), f);
                        assert!(!g.has_vertex_attrs(x));
                    }
                }
            }

            #[test]
            #[should_panic]
            fn unset_vertex_attrs_should_panic() {
                // Test `G::unset_vertex_attrs(i32) -> Result`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.unset_vertex_attrs(&0).unwrap();
            }

            #[test]
            fn has_edge_attrs() {
                // Test `G::get_edge_attrs(i32, i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![((&0, &0), true)],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        vec![((&0, &1), true), ((&1, &2), true), ((&2, &3), true)],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&1, &58), true),
                            ((&3, &75), true),
                            ((&58, &3), true),
                            ((&71, &1), true),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&71, &1), true),
                            ((&1, &58), true),
                            ((&58, &3), true),
                            ((&3, &75), true),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for ((x, y), f) in l {
                        assert_eq!(g.has_edge_attrs(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn has_edge_attrs_should_panic() {
                // Test `G::has_edge_attrs(i32, i32) -> bool`.

                let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.has_edge_attrs(&0, &1);
            }

            #[test]
            fn get_edge_attrs() {
                // Test `G::get_edge_attrs(i32, i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![((&0, &0), Some(&(0.,)))],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        vec![
                            ((&0, &1), Some(&(0.,))),
                            ((&1, &2), Some(&(0.,))),
                            ((&2, &3), Some(&(0.,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&1, &58), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&71, &1), Some(&(0.,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&71, &1), Some(&(0.,))),
                            ((&1, &58), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for ((x, y), f) in l {
                        assert_eq!(g.get_edge_attrs(x, y).ok(), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn get_edge_attrs_should_panic() {
                // Test `G::get_edge_attrs(i32, i32) -> Result`.

                let g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.get_edge_attrs(&0, &1).unwrap();
            }

            #[test]
            fn get_mut_edge_attrs() {
                // Test `G::get_mut_edge_attrs(i32, i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![((&0, &0), Some(&(0.,)))],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        vec![
                            ((&0, &1), Some(&(0.,))),
                            ((&1, &2), Some(&(0.,))),
                            ((&2, &3), Some(&(0.,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&1, &58), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&71, &1), Some(&(0.,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&71, &1), Some(&(0.,))),
                            ((&1, &58), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for ((x, y), f) in l {
                        assert_eq!(g.get_mut_edge_attrs(x, y).ok().copied().as_ref(), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn get_mut_edge_attrs_should_panic() {
                // Test `G::get_mut_edge_attrs(i32, i32) -> Result`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.get_mut_edge_attrs(&0, &1).unwrap();
            }

            #[test]
            fn set_edge_attrs() {
                // Test `G::set_edge_attrs(i32, i32, (f64,))`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![((&0, &0), Some(&(0.,)))],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        vec![
                            ((&0, &1), Some(&(0.,))),
                            ((&1, &2), Some(&(0.,))),
                            ((&2, &3), Some(&(0.,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&1, &58), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&71, &1), Some(&(0.,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&71, &1), Some(&(0.,))),
                            ((&1, &58), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for ((x, y), f) in l {
                        assert_eq!(g.has_edge_attrs(x, y), f.is_some());
                        assert_eq!(g.unset_edge_attrs(x, y).ok().as_ref(), f);
                        assert!(!g.has_edge_attrs(x, y));
                        if let Some(f) = f {
                            g.set_edge_attrs(x, y, f.clone());
                            assert!(g.has_edge_attrs(x, y));
                            assert_eq!(g.get_edge_attrs(x, y).ok(), Some(f));
                        }
                    }
                }
            }

            #[test]
            #[should_panic]
            fn set_edge_attrs_should_panic() {
                // Test `G::set_edge_attrs(i32, i32, (f64, ))`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.set_edge_attrs(&0, &1, (2.,));
            }

            #[test]
            fn unset_edge_attrs() {
                // Test `G::unset_edge_attrs(i32, i32) -> Result`.

                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (None, vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (Some((true,)), vec![(0, (0,))], vec![], vec![]),
                    // ... one vertex and one edge,
                    (
                        Some((true,)),
                        vec![(0, (0,))],
                        vec![((0, 0), (0.,))],
                        vec![((&0, &0), Some(&(0.,)))],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        Some((true,)),
                        vec![(0, (0,)), (1, (0,)), (2, (0,)), (3, (0,))],
                        vec![((0, 1), (0.,)), ((1, 2), (0.,)), ((2, 3), (0.,))],
                        vec![
                            ((&0, &1), Some(&(0.,))),
                            ((&1, &2), Some(&(0.,))),
                            ((&2, &3), Some(&(0.,))),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        Some((true,)),
                        vec![(71, (0,)), (1, (0,)), (58, (0,)), (3, (0,)), (75, (0,))],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&1, &58), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&71, &1), Some(&(0.,))),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        Some((true,)),
                        vec![
                            (35, (0,)),
                            (62, (0,)),
                            (99, (0,)),
                            (29, (0,)),
                            (100, (0,)),
                            (18, (0,)),
                        ],
                        vec![
                            ((71, 1), (0.,)),
                            ((1, 58), (0.,)),
                            ((58, 3), (0.,)),
                            ((3, 75), (0.,)),
                        ],
                        vec![
                            ((&71, &1), Some(&(0.,))),
                            ((&1, &58), Some(&(0.,))),
                            ((&58, &3), Some(&(0.,))),
                            ((&3, &75), Some(&(0.,))),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k, l) in data {
                    let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new_with_attributes(i, j, k);
                    for ((x, y), f) in l {
                        assert!(g.has_edge_attrs(x, y));
                        assert_eq!(g.unset_edge_attrs(x, y).ok().as_ref(), f);
                        assert!(!g.has_edge_attrs(x, y));
                    }
                }
            }

            #[test]
            #[should_panic]
            fn unset_edge_attrs_should_panic() {
                // Test `G::unset_edge_attrs(i32, i32) -> Result`.

                let mut g = $G::<i32, AttributesMap<i32, (bool,), (i64,), (f64,)>>::new(vec![], vec![]);
                g.unset_edge_attrs(&0, &1).unwrap();
            }
        };
    }

    mod attribute_map {
        use grathe::graphs::storages::UndirectedAdjacencyList;
        generic_tests!(UndirectedAdjacencyList);
    }
}
