#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            paste::item! {
                use all_asserts::*;
                use grathe::traits::{Storage, Subgraph};
                use grathe::{E, V};

                #[test]
                #[ignore]
                // FIXME:
                fn subgraph()
                {
                    let g = $G::<i32>::new([], [(0, 1), (0, 2), (1, 2), (2, 3), (3, 3)]);

                    // Build subgraph over 0, 2 and 3.
                    let h = g.subgraph([0, 2, 3]);

                    // Check if it is a subgraph.
                    assert_le!(h, g);

                    // Check if only selected vertices are preserved.
                    assert_true!(V!(h).eq(&[0, 2, 3]));

                    // Check if only associated edges are preserved.
                    assert_true!(E!(h).eq([(&0, &2), (&2, &3), (&3, &3)]));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_subgraph()
                {
                    let g = $G::<i32>::new([], [(0, 1)]);
                    let h = $G::<i32>::new([], [(0, 1), (0, 2)]);

                    assert_le!(g, h);
                    assert_eq!((g <= h), g.is_subgraph(&h));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_supergraph()
                {
                    let g = $G::<i32>::new([], [(0, 1), (0, 2)]);
                    let h = $G::<i32>::new([], [(0, 1)]);

                    assert_ge!(g, h);
                    assert_eq!((g >= h), g.is_supergraph(&h));
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
                use all_asserts::*;
                use grathe::traits::{Storage, Subgraph};
                use grathe::{E, V};

                #[test]
                #[ignore]
                // FIXME:
                fn subgraph()
                {
                    let g = $G::<i32>::new([], [(0, 1), (0, 2), (1, 2), (2, 3), (3, 3)]);

                    // Build subgraph over 0, 2 and 3.
                    let h = g.subgraph([0, 2, 3]);

                    // Check if it is a subgraph.
                    assert_le!(h, g);

                    // Check if only selected vertices are preserved.
                    assert_true!(V!(h).eq(&[0, 2, 3]));

                    // Check if only associated edges are preserved.
                    assert_true!(E!(h).eq([(&0, &2), (&2, &3), (&3, &3)]));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_subgraph()
                {
                    let g = $G::<i32>::new([], [(0, 1)]);
                    let h = $G::<i32>::new([], [(0, 1), (0, 2)]);

                    assert_le!(g, h);
                    assert_eq!((g <= h), g.is_subgraph(&h));
                }

                #[test]
                #[ignore]
                // FIXME:
                fn is_supergraph()
                {
                    let g = $G::<i32>::new([], [(0, 1), (0, 2)]);
                    let h = $G::<i32>::new([], [(0, 1)]);

                    assert_ge!(g, h);
                    assert_eq!((g >= h), g.is_supergraph(&h));
                }
            }
        };
    }

    mod directed_adjacency_list {
        use grathe::graphs::storages::DirectedAdjacencyList;
        generic_tests!(DirectedAdjacencyList);
    }
}
