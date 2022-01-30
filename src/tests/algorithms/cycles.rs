#[cfg(test)]
mod directed {

    macro_rules! generic_tests {
        ($T:ident, $U:ident) => {
            paste::item! {
                #[test]
                #[ignore]
                fn all_cycles() {
                    // Test from Figure 1 of reference paper.
                    let g = $T::<$U>::from_edges(&[
                        (0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (4, 1)
                    ]);
                    let mut search = AllCycles::from(&g);
                    search.run();

                    assert_eq!(
                        search.cycles,
                        [
                            vec![&1, &2, &3, &4, &1],
                            vec![&1, &2, &4, &1]
                        ]
                    );

                    assert_eq!(search.popularity.get(&0), None);
                    assert_eq!(search.popularity[&1], 2);
                    assert_eq!(search.popularity[&2], 2);
                    assert_eq!(search.popularity[&3], 1);
                    assert_eq!(search.popularity[&4], 2);

                    // Test from Figure 9 of reference paper, removed multi-edges.
                    let g = $T::<$U>::from_edges(&[
                        (0, 2),
                        (0, 10),
                        (0, 14),
                        (1, 5),
                        (1, 8),
                        (2, 7),
                        (2, 9),
                        (3, 3),
                        (3, 4),
                        (3, 6),
                        (4, 5),
                        (4, 13),
                        (4, 15),
                        (6, 13),
                        (8, 0),
                        (8, 4),
                        (8, 8),
                        (9, 9),
                        (10, 7),
                        (10, 11),
                        (11, 6),
                        (12, 1),
                        (12, 2),
                        (12, 10),
                        (12, 12),
                        (12, 14),
                        (13, 3),
                        (13, 12),
                        (13, 15),
                        (14, 11),
                        (15, 0),
                    ]);
                    let mut search = AllCycles::from(&g);
                    search.run();

                    assert_eq!(
                        search.cycles,
                        [
                            vec![&0, &10, &11, &6, &13, &3, &4, &15, &0],
                            vec![&0, &10, &11, &6, &13, &12, &1, &8, &0],
                            vec![&0, &10, &11, &6, &13, &12, &1, &8, &4, &15, &0],
                            vec![&0, &10, &11, &6, &13, &15, &0],
                            vec![&0, &14, &11, &6, &13, &3, &4, &15, &0],
                            vec![&0, &14, &11, &6, &13, &12, &1, &8, &0], // <- MISS
                            vec![&0, &14, &11, &6, &13, &12, &1, &8, &4, &15, &0], // <- MISS
                            vec![&0, &14, &11, &6, &13, &15, &0],
                            vec![&1, &8, &4, &13, &12, &1],
                            vec![&3, &3],
                            vec![&3, &4, &13, &3],
                            vec![&3, &6, &13, &3], // <- MISS
                            vec![&6, &13, &12, &10, &11, &6],
                            vec![&6, &13, &12, &14, &11, &6],
                            vec![&8, &8],
                            vec![&9, &9],
                            vec![&12, &12],
                        ]
                    );
                }
            }
        };
    }

    mod adjacency_list_graph {
        use crate::algorithms::AllCycles;
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
                #[ignore]
                fn all_cycles() {
                    let g = $T::<$U>::from_edges(&[
                        (1, 2), (2, 3), (3, 4), (3, 5),
                        (4, 6), (4, 6), (5, 6), (5, 9),
                        (6, 10), (4, 7), (7, 8), (10, 11),
                        (11, 12), (11, 13), (12, 13)
                    ]);
                    let mut search = AllCycles::from(&g);
                    search.run();
                    assert_eq!(
                        search.cycles,
                        [
                            vec![&3, &4, &6, &5, &3],
                            vec![&11, &12, &13, &11]
                        ]
                    );
                }
            }
        };
    }

    mod adjacency_list_graph {
        use crate::algorithms::AllCycles;
        use crate::graphs::UndirectedAdjacencyListGraph;
        use crate::traits::Storage;

        generic_tests!(UndirectedAdjacencyListGraph, i32);
    }
}
