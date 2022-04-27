#[cfg(test)]
mod convert {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use grathe::traits::{Convert, Storage};
                use ndarray::prelude::*;
                use sprs::TriMat;

                #[test]
                fn edge_list() {
                    // Test `G::edge_list() -> EdgeList`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(0, 0)]),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], vec![]),
                        // ... multiple vertices and one edge,
                        (vec![0, 1, 2, 3], vec![(0, 1)], vec![(0, 1)]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(0, 1), (1, 2), (2, 3)],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![(1, 58), (1, 71), (3, 58), (3, 75)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let l = k.clone().into_iter();
                        assert_eq!(
                            g.edge_list(),
                            FromIterator::from_iter(l),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn adjacency_list() {
                    // Test `G::adjacency_list() -> AdjacencyList`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![(0, vec![])]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(0, vec![0])]),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![(0, vec![]), (1, vec![]), (2, vec![]), (3, vec![])],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![(0, vec![1]), (1, vec![0]), (2, vec![]), (3, vec![])],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(0, vec![1]), (1, vec![0, 2]), (2, vec![1, 3]), (3, vec![2])],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (1, vec![58, 71]),
                                (3, vec![58, 75]),
                                (58, vec![1, 3]),
                                (71, vec![1]),
                                (75, vec![3]),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let l = k
                            .clone()
                            .into_iter()
                            .map(|(x, y)| (x, FromIterator::from_iter(y.into_iter())));
                        assert_eq!(
                            g.adjacency_list(),
                            FromIterator::from_iter(l),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn dense_adjacency_matrix() {
                    // Test `G::dense_adjacency_matrix() -> Array2<bool>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], Array2::from_elem((0, 0), false)),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], arr2(&[[false]])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], arr2(&[[true]])),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            arr2(&[
                                [false, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                            ]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            arr2(&[
                                [false, true, false, false],
                                [true, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                            ]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            arr2(&[
                                [false, true, false, false],
                                [true, false, true, false],
                                [false, true, false, true],
                                [false, false, true, false],
                            ]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            arr2(&[
                                [false, false, true, true, false],
                                [false, false, true, false, true],
                                [true, true, false, false, false],
                                [true, false, false, false, false],
                                [false, true, false, false, false],
                            ]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        assert_eq!(
                            g.dense_adjacency_matrix(),
                            k.clone(),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn sparse_adjacency_matrix() {
                    // Test `G::sparse_adjacency_matrix() -> SparseAdjacencyMatrix`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], ((1, 1), vec![], vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            ((1, 1), vec![0], vec![0], vec![true]),
                        ),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], ((4, 4), vec![], vec![], vec![])),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            ((4, 4), vec![0, 1], vec![1, 0], vec![true, true]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                (4, 4),
                                vec![0, 1, 1, 2, 2, 3],
                                vec![1, 0, 2, 1, 3, 2],
                                vec![true, true, true, true, true, true],
                            ),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (
                                (5, 5),
                                vec![0, 2, 0, 3, 1, 2, 1, 4],
                                vec![2, 0, 3, 0, 2, 1, 4, 1],
                                vec![true, true, true, true, true, true, true, true],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let (s, x, y, z) = k.clone();
                        assert_eq!(
                            g.sparse_adjacency_matrix(),
                            TriMat::from_triplets(s, x, y, z),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn dense_incidence_matrix() {
                    // Test `G::dense_incidence_matrix() -> Array2<i8>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], Array2::from_elem((0, 0), 0)),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], Array2::from_elem((1, 0), 0)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], arr2(&[[2]])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], Array2::from_elem((4, 0), 0)),
                        // ... multiple vertices and one edge,
                        (vec![0, 1, 2, 3], vec![(0, 1)], arr2(&[[1], [1], [0], [0]])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            arr2(&[[1, 0, 0], [1, 1, 0], [0, 1, 1], [0, 0, 1]]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            arr2(&[
                                [1, 1, 0, 0],
                                [0, 0, 1, 1],
                                [1, 0, 1, 0],
                                [0, 1, 0, 0],
                                [0, 0, 0, 1],
                            ]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        assert_eq!(
                            g.dense_incidence_matrix(),
                            k.clone(),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn sparse_incidence_matrix() {
                    // Test `G::sparse_incidence_matrix() -> TriMat<i8>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], ((1, 0), vec![], vec![], vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![2])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], ((4, 0), vec![], vec![], vec![])),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            ((4, 1), vec![0, 1], vec![0, 0], vec![1, 1]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                (4, 3),
                                vec![0, 1, 1, 2, 2, 3],
                                vec![0, 0, 1, 1, 2, 2],
                                vec![1, 1, 1, 1, 1, 1],
                            ),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (
                                (5, 4),
                                vec![0, 2, 0, 3, 1, 2, 1, 4],
                                vec![0, 0, 1, 1, 2, 2, 3, 3],
                                vec![1, 1, 1, 1, 1, 1, 1, 1],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let (s, x, y, z) = k.clone();
                        assert_eq!(
                            g.sparse_incidence_matrix(),
                            TriMat::from_triplets(s, x, y, z),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                #[ignore]
                fn dense_marker_matrix() {
                    // FIXME:
                    todo!()
                }

                #[test]
                #[ignore]
                fn sparse_marker_matrix() {
                    // FIXME:
                    todo!()
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
                use grathe::traits::{Convert, Storage};
                use ndarray::prelude::*;
                use sprs::TriMat;

                #[test]
                fn edge_list() {
                    // Test `G::edge_list() -> EdgeList`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(0, 0)]),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], vec![]),
                        // ... multiple vertices and one edge,
                        (vec![0, 1, 2, 3], vec![(0, 1)], vec![(0, 1)]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(0, 1), (1, 2), (2, 3)],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![(1, 58), (3, 75), (58, 3), (71, 1)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let l = k.clone().into_iter();
                        assert_eq!(
                            g.edge_list(),
                            FromIterator::from_iter(l),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn adjacency_list() {
                    // Test `G::adjacency_list() -> AdjacencyList`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], vec![(0, vec![])]),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], vec![(0, vec![0])]),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            vec![(0, vec![]), (1, vec![]), (2, vec![]), (3, vec![])],
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            vec![(0, vec![1]), (1, vec![]), (2, vec![]), (3, vec![])],
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            vec![(0, vec![1]), (1, vec![2]), (2, vec![3]), (3, vec![])],
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            vec![
                                (1, vec![58]),
                                (3, vec![75]),
                                (58, vec![3]),
                                (71, vec![1]),
                                (75, vec![]),
                            ],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let l = k
                            .clone()
                            .into_iter()
                            .map(|(x, y)| (x, FromIterator::from_iter(y.into_iter())));
                        assert_eq!(
                            g.adjacency_list(),
                            FromIterator::from_iter(l),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn dense_adjacency_matrix() {
                    // Test `G::dense_adjacency_matrix() -> Array2<bool>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], Array2::from_elem((0, 0), false)),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], arr2(&[[false]])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], arr2(&[[true]])),
                        // ... multiple vertices and zero edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![],
                            arr2(&[
                                [false, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                            ]),
                        ),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            arr2(&[
                                [false, true, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                                [false, false, false, false],
                            ]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            arr2(&[
                                [false, true, false, false],
                                [false, false, true, false],
                                [false, false, false, true],
                                [false, false, false, false],
                            ]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            arr2(&[
                                [false, false, true, false, false],
                                [false, false, false, false, true],
                                [false, true, false, false, false],
                                [true, false, false, false, false],
                                [false, false, false, false, false],
                            ]),
                        ),
                    ];
                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        assert_eq!(
                            g.dense_adjacency_matrix(),
                            k.clone(),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn sparse_adjacency_matrix() {
                    // Test `G::sparse_adjacency_matrix() -> SparseAdjacencyMatrix`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], ((1, 1), vec![], vec![], vec![])),
                        // ... one vertex and one edge,
                        (
                            vec![0],
                            vec![(0, 0)],
                            ((1, 1), vec![0], vec![0], vec![true]),
                        ),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], ((4, 4), vec![], vec![], vec![])),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            ((4, 4), vec![0], vec![1], vec![true]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            ((4, 4), vec![0, 1, 2], vec![1, 2, 3], vec![true, true, true]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (
                                (5, 5),
                                vec![0, 1, 2, 3],
                                vec![2, 4, 1, 0],
                                vec![true, true, true, true],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let (s, x, y, z) = k.clone();
                        assert_eq!(
                            g.sparse_adjacency_matrix(),
                            TriMat::from_triplets(s, x, y, z),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn dense_incidence_matrix() {
                    // Test `G::dense_incidence_matrix() -> Array2<i8>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], Array2::from_elem((0, 0), 0)),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], Array2::from_elem((1, 0), 0)),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], arr2(&[[0]])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], Array2::from_elem((4, 0), 0)),
                        // ... multiple vertices and one edge,
                        (vec![0, 1, 2, 3], vec![(0, 1)], arr2(&[[-1], [1], [0], [0]])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            arr2(&[[-1, 0, 0], [1, -1, 0], [0, 1, -1], [0, 0, 1]]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            arr2(&[
                                [-1, 0, 0, 1],
                                [0, -1, 1, 0],
                                [1, 0, -1, 0],
                                [0, 0, 0, -1],
                                [0, 1, 0, 0],
                            ]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        assert_eq!(
                            g.dense_incidence_matrix(),
                            k.clone(),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                fn sparse_incidence_matrix() {
                    // Test `G::sparse_incidence_matrix() -> TriMat<i8>`.

                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec![0], vec![], ((1, 0), vec![], vec![], vec![])),
                        // ... one vertex and one edge,
                        (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![0])),
                        // ... multiple vertices and zero edges,
                        (vec![0, 1, 2, 3], vec![], ((4, 0), vec![], vec![], vec![])),
                        // ... multiple vertices and one edge,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1)],
                            ((4, 1), vec![0, 1], vec![0, 0], vec![-1, 1]),
                        ),
                        // ... multiple vertices and multiple edges,
                        (
                            vec![0, 1, 2, 3],
                            vec![(0, 1), (1, 2), (2, 3)],
                            (
                                (4, 3),
                                vec![0, 1, 1, 2, 2, 3],
                                vec![0, 0, 1, 1, 2, 2],
                                vec![-1, 1, -1, 1, -1, 1],
                            ),
                        ),
                        // ... random vertices and edges,
                        (
                            vec![71, 1, 58, 3, 75],
                            vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                            (
                                (5, 4),
                                vec![0, 2, 1, 4, 2, 1, 3, 0],
                                vec![0, 0, 1, 1, 2, 2, 3, 3],
                                vec![-1, 1, -1, 1, -1, 1, -1, 1],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let g = $G::<i32>::new(i.clone(), j.clone());
                        let (s, x, y, z) = k.clone();
                        assert_eq!(
                            g.sparse_incidence_matrix(),
                            TriMat::from_triplets(s, x, y, z),
                            "with test data: '{:?}'",
                            (i, j, k)
                        );
                    }
                }

                #[test]
                #[ignore]
                fn dense_marker_matrix() {
                    // FIXME:
                    todo!()
                }

                #[test]
                #[ignore]
                fn sparse_marker_matrix() {
                    // FIXME:
                    todo!()
                }
            };
        }

        mod directed_adjacency_list {
            use grathe::graphs::storages::DirectedAdjacencyList;
            generic_tests!(DirectedAdjacencyList);
        }
    }
}
