#[cfg(test)]
mod directed {

    macro_rules! generic_tests {
        ($T:ident, $U:ident) => {
            paste::item! {
                #[test]
                fn all_simple_paths() {
                    let g = $T::<$U>::from_edges(&[
                        (0, 1), (0, 2), (0, 3),
                        (1, 2), (1, 3),
                        (2, 3)
                    ]);
                    let mut search = AllSimplePaths::from((&g, &0, &3));
                    search.run();
                    assert_eq!(
                        search.simple_paths,
                        [
                            vec![&0, &1, &2, &3],
                            vec![&0, &1, &3],
                            vec![&0, &2, &3],
                            vec![&0, &3],
                        ]
                    );
                }

                #[test]
                #[should_panic]
                fn all_simple_paths_should_panic()
                {
                    // Build a null graph.
                    let g = $T::<$U>::new();
                    AllSimplePaths::from((&g, &0, &1)).run();
                }
            }
        };
    }

    mod adjacency_list_graph {
        use crate::algorithms::AllSimplePaths;
        use crate::graphs::DirectedAdjacencyListGraph;
        use crate::traits::Storage;

        generic_tests!(DirectedAdjacencyListGraph, i32);
    }
}

#[cfg(test)]
mod undirected {

    macro_rules! generic_tests {
        ($T:ident, $U:ident) => {
            paste::item! {
                #[test]
                fn all_simple_paths() {
                    let g = $T::<$U>::from_edges(&[
                        (0, 1), (0, 2), (0, 3),
                        (1, 2), (1, 3),
                        (2, 3)
                    ]);
                    let mut search = AllSimplePaths::from((&g, &0, &3));
                    search.run();
                    assert_eq!(
                        search.simple_paths,
                        [
                            vec![&0, &1, &2, &3],
                            vec![&0, &1, &3],
                            vec![&0, &2, &1, &3],
                            vec![&0, &2, &3],
                            vec![&0, &3],
                        ]
                    );
                }

                #[test]
                #[should_panic]
                fn all_simple_paths_should_panic()
                {
                    // Build a null graph.
                    let g = $T::<$U>::new();
                    AllSimplePaths::from((&g, &0, &1)).run();
                }
            }
        };
    }

    mod adjacency_list_graph {
        use crate::algorithms::AllSimplePaths;
        use crate::graphs::UndirectedAdjacencyListGraph;
        use crate::traits::Storage;

        generic_tests!(UndirectedAdjacencyListGraph, i32);
    }
}
